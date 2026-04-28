# List-row, result-row, and card-row identity contract

This document freezes the shared row anatomy used by list-backed
surfaces before each surface invents a private version of identity,
freshness, provider/source, status, and quick actions. It applies to
triage rows, review queues, provider inventories, run history,
incident queues, search/result rows, activity feeds, sidebars, start
surfaces, dense tables, and companion summaries.

Machine-readable companions:

- [`/schemas/ux/list_row.schema.json`](../../schemas/ux/list_row.schema.json)
  defines `list_row_record`, `result_row_record`, and
  `card_row_record`.
- [`/fixtures/ux/list_row_cases/`](../../fixtures/ux/list_row_cases/)
  contains worked rows for blocked, stale, provider-backed,
  local-only, duplicate-merged, and card-promoted cases.

This contract composes with:

- [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  for filter, sort, selection, count truth, and batch-review scope.
- [`/docs/ux/selection_and_scope_contract.md`](./selection_and_scope_contract.md)
  for focus, current item, selected item, activation, and range
  behavior.
- [`/docs/ux/search_result_contract.md`](./search_result_contract.md)
  for search-specific result truth, rank reasons, and replace scope.
- [`/docs/ux/recent_work_and_restore_card_contract.md`](./recent_work_and_restore_card_contract.md)
  for recent-work and restore rows.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  for cross-surface freshness labels.
- [`/docs/ux/chronology_row_contract.md`](./chronology_row_contract.md)
  for event/history rows after row state becomes timeline evidence.

If a more specific source contract owns a deeper object shape, this
row contract projects that object rather than replacing it.

## Scope

Frozen here:

- shared identity fields for canonical object id, title or primary
  label, source/provider, freshness or age, owner or actor, row state,
  quick actions, and open-details behavior;
- one closed state vocabulary that keeps unread, draft, blocked,
  stale, failed, and healthy distinct;
- when a terse row promotes to an expanded row, card, or tile;
- which metadata may move behind tooltip, hovercard, peek, or full
  details;
- compact versus expanded behavior for dense tables, sidebars, start
  surfaces, and mobile/companion summaries;
- one strong left scan edge, reason-chip placement, duplicate-merge
  behavior, and a right-side utility budget; and
- quick-action disclosure rules for target identity, authority, scope,
  and freshness.

Out of scope:

- final visual styling for every card family;
- search rank scoring or provider-specific APIs;
- table column management beyond row-level utility limits; and
- implementing any renderer.

## Shared Row Anatomy

Every row record carries these groups.

| Group | Required contents |
| --- | --- |
| Record identity | `record_kind`, schema version, row id, surface family, presentation class, minted time |
| Canonical object | object kind, canonical object id, object revision or snapshot ref, optional route ref |
| Display identity | title or primary label, optional secondary label, one left-edge anchor |
| Source/provider | source kind, source label, source ref, authority class, optional provider account or host label |
| Freshness/age | freshness class, observed time, age label, stale reason where relevant |
| Owner/actor | owner or actor kind, redaction-aware label, actor ref |
| Row state | one `state_class`, status intent, user label, and reason chips |
| Quick actions | typed actions with target identity, scope, authority, freshness, and disclosure blocks |
| Details route | default action, route ref, activation model, context preservation, revalidation behavior |
| Density and layout | visible slots, hidden-until-peek slots, left scan edge, right utility budget |
| Duplicate merge | merge state, canonical merge key, merged members, provenance visibility |
| Peek policy | what hover/peek may reveal and what it must never hide |
| Accessibility | focus model, keyboard default action, selection role, announcement fields |

Rows may omit optional detail, snippets, thumbnails, metrics, or
secondary badges. They may not omit canonical identity, state, source,
freshness, authority disclosure for actions, or the open-details route.

## State Vocabulary

The `state_class` set is deliberately small and not interchangeable.

| State | Meaning | Required row behavior |
| --- | --- | --- |
| `healthy` | The row is usable for its stated scope and freshness. | No warning styling is required, but source and freshness still render or remain one-step inspectable. |
| `unread` | The row needs user attention because it has not been acknowledged or reviewed. | Mark as attention state without implying failure, staleness, or blockage. |
| `draft` | The row represents local, unsent, unpublished, or not-yet-provider-accepted work. | Show local-only or pending-publish status and keep provider mutation separate from local save. |
| `blocked` | The row or at least one consequential action is denied by policy, trust, permission, missing dependency, or authority. | Include a blocked reason chip and keep safe inspect/export routes available where permitted. |
| `stale` | The row is based on old, cached, imported, or no-longer-current data. | Show freshness age and stale reason before quick actions. Mutating actions must revalidate or route through review. |
| `failed` | The represented run, check, sync, publish, or operation failed. | Show failure source and recovery path without collapsing into blocked unless authority denies recovery. |

`blocked`, `stale`, and `failed` are separate axes. A stale provider
row is not automatically failed. A failed run is not automatically
blocked. A blocked publish action may still leave a healthy local
draft row inspectable.

## Source and Freshness

The row source block answers who or what produced the row:

- `local` for first-party local state such as workspace, buffer,
  local draft, or local run metadata;
- `provider` for connected services or provider-hosted indexes;
- `remote_runtime` for remote agents, containers, debug targets, or
  runtime sessions;
- `generated` for generated or derived artifacts;
- `imported` for scans, bundles, snapshots, or support evidence;
- `mirrored` for local mirrors of external sources; and
- `mixed_merged` when duplicate rows from multiple sources collapsed
  into one row.

The authority class is independent from source. A provider-backed row
may be inspect-only, provider-authoritative, locally drafted, imported,
or mixed-authority after duplicate merge.

Freshness labels must say whether the row is live, fresh, recently
stale, materially stale, a cached snapshot, an imported snapshot,
local-only unsynced, or unknown. Rows with `recently_stale`,
`materially_stale`, `cached_snapshot`, `imported_snapshot`,
`local_only_unsynced`, or `unknown` carry a reason chip or secondary
freshness line before the utility area.

## Quick Actions

Quick actions are compact command projections. They are not private
buttons.

Every quick action carries:

- action id and action kind;
- target object id, target label, and target object kind;
- scope class and scope label;
- authority class, authority label, approval requirement, and whether
  gates revalidate on invoke;
- freshness class and whether freshness must be refreshed before
  invoke;
- effect class: read-only, local state change, provider mutation,
  remote execution, destructive metadata cleanup, or export/share;
- enabled state and disabled reason if disabled; and
- disclosure booleans proving target identity, authority, and
  freshness are visible in the row, overflow menu, or immediate
  review sheet.

Rules:

1. A quick action must never hide the target identity it will affect.
2. A quick action must never hide whether it has local, provider,
   remote, policy, or mixed authority.
3. A quick action must never hide stale, partial, cached, imported, or
   local-only freshness.
4. Read-only inspect routes can remain enabled on blocked rows when
   policy allows inspection.
5. Provider mutations, destructive metadata cleanup, remote execution,
   and export/share actions open a review, confirmation, or authority
   sheet when the row is stale, blocked, partial, provider-limited, or
   local-only.
6. Overflow menus may hide low-frequency actions, not identity,
   authority, freshness, or the safest open-details path.

## Open Details Behavior

Every row has one open-details route. `Enter`, double-click, touch
activation, or the primary open action must resolve to that route or
to a required review/repair path before continuing.

The route preserves:

- the canonical object id;
- the current collection context and selection where applicable;
- the source/provider identity;
- the freshness class at the time the route was minted;
- any duplicate-merge membership; and
- the fallback route when the target vanished, became blocked, or
  needs revalidation.

A row may open a peek first for dense review, but the peek must expose
the same identity, authority, and freshness truth as the row.

## Row to Card Promotion

A row promotes to an expanded row, card, or tile when the terse row
cannot preserve identity and state without hiding consequential truth.

Promotion is required when any of these are true:

- the object has multiple targets or merged provider members that need
  comparison;
- recovery or approval requires multi-step review;
- the row needs a media, diff, evidence, or preview area;
- blocked, failed, partial, or stale detail needs more than one short
  reason chip;
- mobile or companion summary would otherwise hide the target identity
  or authority; or
- the row represents a starter, incident, run, or restore summary with
  counts that must remain separated.

Promotion is optional when richer layout is only comfort or browsing
polish. It is not allowed merely to hide row-required metadata inside
the card.

## Peek and Hover Metadata

Tooltip, hovercard, peek, and full-detail layers form an escalation
ladder:

| Layer | May contain | Must not contain exclusively |
| --- | --- | --- |
| Tooltip | short clarification, full truncated label | canonical object id, state, authority, freshness |
| Hovercard | extra provenance, owner, timestamps, duplicate members | only copy of stale, blocked, failed, draft, or provider authority state |
| Peek | preview, snippet, diff, evidence, row-neighbor context | only path to a required quick action or default open route |
| Full view | full object, audit, history, raw-safe details | no special exception; still preserves row identity |

Metadata may move behind peek only after the row keeps a scannable
primary label, source/provider, freshness, row state, and safest
open-details path visible or keyboard-reachable in one step.

## Density Rules

### Compact dense table

- Required visible slots: left-edge anchor, title, state, source or
  provider, freshness/age, and one primary open/details action.
- Reason chips appear immediately after the title or before quick
  actions.
- Maximum visible quick actions: two.
- Maximum visible status chips: two.
- Overflow is required once the budget is exceeded.
- Selection, focus, and activation remain separate.

### Standard list or queue

- Required visible slots: left-edge anchor, title, secondary label,
  state, source/provider, freshness/age, owner/actor, primary action,
  and overflow.
- Maximum visible quick actions: three.
- Maximum visible status chips: three.
- Secondary metadata can wrap or move below the primary line before
  identity truncates.

### Expanded row

- May show snippets, short evidence, duplicate-source summaries,
  previews, or recovery hints inline.
- Still keeps one left scan edge.
- Quick actions do not migrate between left identity and reason chips.

### Sidebar

- Required visible slots: left-edge anchor, title, compact state cue,
  and freshness/source cue when stale, blocked, draft, failed,
  provider-backed, or local-only.
- Owner, duplicate members, and low-frequency actions move to peek or
  overflow.
- Default activation remains discoverable by keyboard.

### Start surface

- Required visible slots: title, source/target, freshness or last
  validation, trust/authority state, and safe open/recovery action.
- Remove/cleanup actions are secondary or destructive and must not sit
  between identity and safe open.

### Mobile or companion summary

- Required visible slots: title, state, source/provider, freshness, and
  one safe open/details route.
- Provider mutations, destructive actions, and exports open a review
  surface. They do not execute from a hidden swipe action.
- Hover-only disclosure is non-conforming; use tap-to-peek or details.

## Left Scan Edge and Utility Budget

Rows keep one strong left edge:

1. selection/focus affordance where present;
2. stable icon or object-kind marker;
3. title or primary label;
4. secondary identity; and
5. reason chips before right-side utilities.

Right-side utilities are bounded. A row may show at most three visible
quick actions and at most three visible status chips in standard or
expanded density, with lower limits in compact and sidebar density.
Overflow uses a menu or detail surface that preserves target,
authority, and freshness disclosure.

The right edge is for utilities, not identity. Identity, freshness,
blocked state, or provider authority cannot be pushed wholly into the
right utility area.

## Duplicate Merge Behavior

When multiple sources produce the same canonical object, the surface
collapses them only when it can prove the merge key and preserve
provenance.

Merged rows carry:

- canonical merge key;
- member count;
- member refs with source label, freshness class, and merge role;
- conflict or divergence note where providers disagree;
- a reason chip such as provider-backed, merged duplicate, partial, or
  stale when relevant; and
- a detail or peek path that lists members.

Rows must not merge when:

- targets are only visually similar;
- source authority differs in a way that changes action scope;
- freshness classes conflict and cannot be explained;
- one member is blocked or hidden and the merged row would erase that;
  or
- provider identity is required for audit or support and cannot be
  preserved.

## Fixture Acceptance

Fixtures are conforming when a reviewer can tell, without surface-
specific jargon:

- what canonical object the row represents;
- whether it is unread, draft, blocked, stale, failed, or healthy;
- whether it is local, provider-backed, remote, generated, imported,
  mirrored, or mixed;
- how fresh it is and why stale, cached, imported, partial, or
  local-only state exists;
- who owns or acted on it;
- which quick actions are safe, disabled, provider-mutating,
  destructive, export-bearing, or inspect-only;
- what opens on default activation;
- whether duplicates were merged and where member provenance lives;
- why the row stays compact or promotes to a richer card; and
- which metadata moves to hover, peek, or full detail.
