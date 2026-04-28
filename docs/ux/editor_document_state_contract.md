# Editor Document State Contract

This contract freezes the document-header, context, status, compare,
and preview vocabulary used when an editor is showing something other
than ordinary writable source. It keeps document identity and editability
honest across tabs, breadcrumbs, status surfaces, compare sheets,
preview sheets, support bundles, accessibility labels, and documentation
screenshots.

Companion artifacts:

- [`/schemas/editor/document_state_badge.schema.json`](../../schemas/editor/document_state_badge.schema.json)
  defines the machine-readable state-badge catalog and document-state
  projection records.
- [`/fixtures/editor/document_state_cases/`](../../fixtures/editor/document_state_cases/)
  contains worked cases for recovered read-only snapshots, stale
  generated output, and compare views whose source side is dirty.

This contract composes with:

- [`/docs/ux/editor_anatomy_contract.md`](./editor_anatomy_contract.md)
  for canonical editor layers and the rule that constrained editing
  cannot be icon-only.
- [`/docs/ux/tabs_editor_groups_contract.md`](./tabs_editor_groups_contract.md)
  for tab, editor-group, overflow, restore, compare, and preview
  identity.
- [`/docs/ux/breadcrumb_contract.md`](./breadcrumb_contract.md)
  for breadcrumb projection, overflow, keyboard access, and stale or
  unavailable context chips.
- [`/docs/ux/status_strip_family_contract.md`](./status_strip_family_contract.md)
  and [`/docs/ux/status_bar_contract.md`](./status_bar_contract.md)
  for compact status placement and details routing.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  for generated, mirrored, imported, preview, and notebook/result
  artifact edit posture.

If this document conflicts with a source product document in `.t2/docs/`,
that source wins and this contract, schema, and fixtures must update in
the same change.

## Boundary

Document-state badges are editor chrome and export metadata. They do not
own buffer bytes, save authority, policy decisions, generator lineage,
preview runtime authority, or compare algorithms. They project those
owners into one stable vocabulary that users and support tooling can
read without guessing.

Frozen here:

- canonical document state names and badge classes;
- required placement for each state across tabs, document headers,
  breadcrumb/context rows, status surfaces, and compare or preview
  sheets;
- recovery and source-navigation actions for constrained states;
- identity-preservation rules across splits, restores, compare mode, and
  transient-preview promotion; and
- accessibility, screenshot, and support-export naming rules.

Out of scope: final visual styling, icon artwork, every future document
provider, provider-specific copy, and concrete command implementation.

## Canonical States

The state name and badge class MUST match exactly in schemas, support
bundles, accessibility labels, docs screenshots, and component tests.
Surfaces may localize visible prose, but they may not mint local state
names.

| State / badge class | Meaning | Required primary action family |
| --- | --- | --- |
| `dirty` | The document has unsaved or unapplied local change authority. | `compare_against_source`, save / discard through the owning save contract |
| `pinned` | The tab is retained in the working set and close behavior is narrowed. | `pin` / `unpin` through the tab contract |
| `compare` | The view is part of a compare surface with a source, target, base, result, or unified role. | `compare_against_source` or role-specific compare navigation |
| `recovered_snapshot` | The document is restored from a journal, checkpoint, crash recovery, or evidence packet rather than opened as live source. | `inspect_restore_provenance`, `reopen_writable_source`, `compare_against_source` |
| `generated_output` | The visible artifact is derived from canonical source, a generator, resolver, kernel, or preview runtime. | `open_source`, `compare_against_source`, `regenerate_review` |
| `imported_patch` | The visible change arrived from import, replay, handoff, or external patch context and is not yet local source authority. | `review_imported_patch`, `compare_against_source` |
| `read_only` | The current surface can inspect/copy but cannot directly write the visible document. | `reopen_writable_source` or `open_source` when available |
| `mirrored` | The visible artifact is a mirror or promoted copy of upstream truth. | `inspect_mirror_provenance`, `compare_against_source` |
| `policy_locked` | A policy or entitlement decision narrows editing, regeneration, publication, or share behavior. | `inspect_policy` |
| `live_preview` | The view is a runtime projection or live preview, not source authority. | `open_source`, `reconnect_preview`, `regenerate_review` when stale |
| `conflict` | Save, compare, restore, or merge authority disagrees and a decision is required. | `resolve_conflict`, `compare_against_source` |
| `stale` | The visible projection or derived artifact is behind its source, runtime, mirror, policy, or generation basis. | `regenerate_review`, refresh/rebind through the owning subsystem |

`clean`, `writable`, and `current` are default postures, not badge
classes. They may appear in details or support exports, but they do not
replace the state badges above when a non-default condition is active.

## Surface Placement

State that affects identity, editability, source of truth, review basis,
or recovery MUST appear as text or structure. Color, a tiny icon, a
tooltip, or a status-only item is never enough.

| State | Tab / document header | Breadcrumb / context | Status surface | Compare or preview sheet |
| --- | --- | --- | --- | --- |
| `dirty` | Required; names modified/conflict/recovered-draft authority. | Required when the dirty authority is not ordinary source. | Required for active save-blocked or conflict cases. | Required before close, save, compare, or apply review. |
| `pinned` | Required; structural pinned section or text label. | Optional. | Not required. | Required only when a sheet changes close/unpin behavior. |
| `compare` | Required; names compare role. | Required; names basis and counterpart. | Optional compact role/basis item. | Required; role labels cannot be icons alone. |
| `recovered_snapshot` | Required; names recovered, restored, or evidence-only posture. | Required; names provenance and live-source relation. | Required until the user accepts, discards, or reopens source. | Required for compare-to-source or restore-review flows. |
| `generated_output` | Required; says generated, output, or derived. | Required; names canonical source or generator relation. | Required when stale, blocked, or edit posture differs from source. | Required for generated compare, regenerate, or preview review. |
| `imported_patch` | Required; says imported patch or replayed patch. | Required; names import source and target authority. | Optional unless apply is blocked or stale. | Required before apply or merge. |
| `read_only` | Required; says read-only or inspect-only. | Required; names reason and writable-source path if known. | Required while active. | Required when a write action is blocked. |
| `mirrored` | Required when the mirror can be mistaken for local source. | Required; names mirror/upstream relation and freshness. | Required when cached, stale, or policy-owned. | Required for mirror drift or replace-by-promotion review. |
| `policy_locked` | Required; says policy locked or managed lock. | Required; names policy source or epoch ref. | Required while any primary action is narrowed. | Required for blocked write, publish, regenerate, share, or export actions. |
| `live_preview` | Required; says preview, live, reconnecting, stale snapshot, or manual rebind. | Required; names runtime/source relation. | Required; names connection/freshness posture. | Required for preview, visual edit, stale snapshot, or rebind review. |
| `conflict` | Required; names conflict class. | Required; names conflicting authority or basis. | Required while unresolved. | Required; conflict resolution cannot hide source/target/base labels. |
| `stale` | Required when attached to generated, preview, mirror, recovered, or compare state. | Required; names stale basis. | Required while active. | Required when the user reviews, regenerates, or compares the stale artifact. |

Forbidden placement:

- showing only an icon, colored dot, or italic label for any state above;
- burying read-only, generated, recovered, policy-locked, or conflict
  reasons in hover-only detail;
- saying "preview", "generated", or "restored" in a tab while the
  context row omits the source, generator, mirror, runtime, or restore
  provenance;
- letting a status bar item be the only place where editability changed;
  and
- dropping active state badges from overflow rows, support exports, docs
  screenshots, or accessibility labels.

## Composite States

Composite states are normal and MUST preserve every active axis.

| Combination | Required behavior |
| --- | --- |
| `recovered_snapshot` + `read_only` | Header says recovered read-only. Context names restore provenance and writable-source path. Actions include inspect provenance and reopen writable source when possible. |
| `generated_output` + `stale` | Header says generated stale. Context names canonical source/generator and stale basis. Actions include open source, compare against source, and regenerate review. |
| `compare` + `dirty` | Compare role labels remain visible and the dirty side names its dirty authority. The compare summary says whether the basis includes unsaved source or a saved snapshot. |
| `generated_output` + `dirty` | Dirty authority must be generated-artifact, override, recovered-draft, or conflict authority; ordinary source dirty must not be implied. |
| `live_preview` + `recovered_snapshot` | Restored live surfaces start as evidence-only, stale snapshot, or manual rebind until runtime authority is revalidated. |
| `policy_locked` + `read_only` | Policy source and read-only reason both remain visible. Policy lock does not replace the read-only label. |
| `mirrored` + `stale` | Mirror freshness and upstream relation both remain visible; the recovery path is refresh/promotion review, not ad hoc local edit. |

## Canonical Actions

Document-state projections carry command-backed actions. Labels may vary
by product surface, but action classes are stable.

| Action class | Used for | Required behavior |
| --- | --- | --- |
| `open_source` | Generated, mirrored, preview, or imported views with a known canonical source. | Opens or focuses the canonical source without implying the derived artifact became editable. |
| `compare_against_source` | Generated, recovered, mirrored, imported, dirty, stale, or conflict cases. | Names compared roles and basis snapshot before rendering. |
| `regenerate_review` | Stale generated output, stale preview projections, generator drift, or rebuild-required artifacts. | Opens a reviewable regeneration plan before replacing visible output. |
| `reopen_writable_source` | Recovered read-only snapshots, read-only mirrors, read-only previews, or policy-narrowed copies when a writable source is permitted. | Opens writable authority as a distinct source surface and preserves the snapshot tab until the user closes it. |
| `inspect_restore_provenance` | Any recovered snapshot, recovered draft, checkpoint, or evidence-only restore. | Shows restore class, source journal/checkpoint, capture time, redaction posture, and safe next actions. |
| `review_imported_patch` | Imported or replayed patch artifacts. | Shows import origin, target authority, apply scope, conflict posture, and checkpoint/undo class. |
| `inspect_policy` | Policy-locked or entitlement-narrowed documents. | Shows policy source, epoch/ref, affected actions, and escalation/export route. |
| `inspect_mirror_provenance` | Mirrored documents and mirrored generated artifacts. | Shows upstream owner, mirror basis, freshness, and refresh/promotion route. |
| `reconnect_preview` | Live-preview or preview-snapshot documents. | Revalidates runtime authority and source mapping before claiming live state. |
| `resolve_conflict` | Save, merge, restore, compare, or authority conflicts. | Opens a compare/review surface that names sides without ambiguous "ours/theirs" language. |

Every action record MUST declare whether it preserves the current
document identity, opens a related source identity, or mints a new
durable document identity.

## Identity Preservation

Document state travels with logical identity, not with pixels.

1. **Splits and moves.** Moving or splitting a tab preserves
   `stable_document_ref`, `stable_content_ref`, source authority refs,
   active state badges, dirty authority, read-only reason, generated
   lineage, compare basis, and restore provenance. A new view may get a
   new pane or group ref, but it cannot mint a new document truth.
2. **Restores.** Restore rehydrates layout and document-state badges
   before heavy providers run. A recovered snapshot remains a snapshot
   until the user explicitly reopens writable source or accepts a
   restore action.
3. **Compare mode.** Compare presentation adds `compare` and a
   `compare_role`; it does not erase dirty, recovered, generated,
   read-only, stale, policy-locked, or conflict badges on either side.
4. **Transient preview promotion.** Promoting a transient preview to a
   durable editor tab preserves the transient preview ref, source refs,
   freshness, runtime posture, and accessible state text. The durable tab
   does not claim live runtime authority until the runtime revalidates.
5. **Support export.** Support bundles preserve state names, badge
   classes, surface placements, action classes, and authority refs. They
   do not need raw file bodies, raw paths, raw URLs, raw preview DOM, raw
   provider payloads, or secret material to explain document state.

## Accessibility, Screenshots, and Support

Every active state badge MUST have:

- a short visible label suitable for the tab/header;
- a longer accessible label naming state, reason, and primary action
  when action exists;
- screenshot-safe text that survives documentation crops and high
  contrast;
- support-export fields for state name, badge class, authority refs,
  and action refs; and
- keyboard-reachable details when the state affects editing,
  recoverability, trust, policy, or source-of-truth interpretation.

Localization may translate labels but MUST keep machine keys stable.
Docs screenshots may use captions, callouts, or expanded overflow rows,
but the visible UI in the screenshot must still show the same state
names or localized equivalents.

## Schema and Fixture Requirements

Records using
[`/schemas/editor/document_state_badge.schema.json`](../../schemas/editor/document_state_badge.schema.json)
must satisfy these invariants:

1. `state_class` and `badge_class` use the canonical vocabulary above.
2. Every active state has at least one placement on a tab or document
   header and at least one placement on a breadcrumb/context, status,
   compare, or preview surface according to the placement table.
3. `icon_only_forbidden` is true for every state placement.
4. Every active non-default editability state carries an authority,
   provenance, policy, source, generator, runtime, mirror, import, or
   conflict ref.
5. Every canonical action declares its action class, command ref, target
   relation, and identity-preservation behavior.
6. Identity-preservation records cover the transitions exercised by the
   fixture: split/move, restore, compare, or transient-to-durable
   handoff.

## Acceptance Checklist

A reviewer can accept an implementation, fixture, support export, or
docs screenshot when:

1. Source editing, generated preview, compare review, recovered snapshot
   inspection, read-only browsing, mirrored browsing, policy-locked
   browsing, and live-preview runtime projection are distinguishable
   from editor chrome alone.
2. Tabs, headers, breadcrumbs/context, status surfaces, and compare or
   preview sheets all use the same state names and badge classes.
3. Conflicting combinations preserve every active state instead of
   collapsing to the most visually convenient badge.
4. Recovery and source-navigation actions are explicit and
   command-backed.
5. Identity survives split, restore, compare, and transient-preview
   promotion without converting snapshots, previews, or generated output
   into writable source by implication.
