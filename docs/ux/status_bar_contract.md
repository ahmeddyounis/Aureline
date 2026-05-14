# Status bar item contract

This contract freezes Aureline's status bar as an ambient
instrumentation surface. It defines which state may enter the bar,
how status items are prioritized, how crowded layouts overflow, and
what extension contributors must provide before a status item is
admitted.

Machine-readable companions:

- [`/schemas/ux/status_item.schema.json`](../../schemas/ux/status_item.schema.json)
  defines catalog, item, layout snapshot, and extension-budget decision
  records for status bar conformance.
- [`/fixtures/ux/status_items/`](../../fixtures/ux/status_items/)
  contains worked examples for the priority ladder, recovery-critical
  visibility, compact overflow parity, and extension contribution
  review.

This contract composes with, and does not replace:

- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  for the required `status_bar` shell zone, adaptive classes, compact
  shell fallback, and keyboard-reachable collapse rules.
- [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  for `route.status_bar_link` and the rule that status links open the
  narrowest useful owner surface.
- [`/docs/commands/command_graph_and_ui_slots_seed.md`](../commands/command_graph_and_ui_slots_seed.md)
  for `status_bar.primary_status_item` and `status_bar.quick_action`
  slot keys.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  for command descriptors, disabled reasons, help anchors, and command
  parity.
- [`/docs/ux/title_context_bar_contract.md`](./title_context_bar_contract.md)
  for workspace, trust, host, profile, route, and degraded/recovery
  truth projected into the workspace status item.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  and [`/docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md)
  for durable event lineage and notification routing.
- [`/docs/extensions/runtime_budget_packet.md`](../extensions/runtime_budget_packet.md)
  for extension runtime, activation, and quarantine visibility.
- [`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  for transport posture and environment status strip semantics.
- [`/docs/ux/state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md)
  for degraded and recovery vocabulary.

If this document conflicts with the source UI/UX spec, shell-zone
contract, navigation contract, command contract, title/context contract,
or extension runtime budget contract, the upstream source wins and this
document plus its schema and fixtures must update in the same change.

## Scope

This contract freezes:

- status item classes and priority order;
- stable-slot expectations for high-frequency status;
- compact-width overflow rules and status-menu parity;
- action behavior for visible, overflowed, and hidden items;
- extension contribution budgets and anti-branding guardrails;
- anti-jitter rules for spinners, counters, and rapidly changing values.

Out of scope: final icons, color tokens, typography, platform-native
status bar widgets, and the eventual renderer or shell crate types.

## Status item classes

Every status item belongs to exactly one class. Classes are ordered.
Lower-priority classes move to overflow before higher-priority classes.

| Order | Class | Purpose | Examples | Overflow posture |
| ---: | --- | --- | --- | --- |
| 1 | `recovery_critical` | State that can block, lose, narrow, or misrepresent core work. | save failed, restricted workspace, policy block, remote read-only degraded, rollback required | Claims a stable visible recovery slot at every supported width. |
| 2 | `active_context_truth` | Current execution, trust, host, branch, profile, route, or source-fidelity truth that changes command meaning. | execution target, workspace trust, remote host, active branch, profile, line-ending conversion review | Stable cluster; condenses before overflow. |
| 3 | `ongoing_work` | Long-running or repeated work that should remain inspectable without toast spam. | indexing, tests, sync, downloads, AI task, extension lint run | Aggregates first, then overflows behind the same status menu row. |
| 4 | `ambient_metadata` | Low-risk editor or environment facts useful for glanceable inspection. | cursor position, indentation, encoding, language mode, ordinary line endings | Overflows first and may be hidden until applicable. |

Promotion rule: an ambient or work item is promoted only when the
underlying state becomes consequence-bearing. For example, ordinary
encoding is ambient metadata; decode failure or pending lossy conversion
is recovery-critical.

## Priority ladder

Priority rank bands are frozen:

| Band | Rank range | Class |
| --- | ---: | --- |
| `recovery_critical_first` | 0-99 | `recovery_critical` |
| `active_context_truth_second` | 100-199 | `active_context_truth` |
| `ongoing_work_third` | 200-299 | `ongoing_work` |
| `ambient_metadata_fourth` | 300-399 | `ambient_metadata` |

Rules:

1. Recovery-critical state cannot be displaced by extension branding,
   promotional items, vanity counters, or ordinary ambient metadata.
2. Severe states claim `status.slot.recovery.primary` or a shell-owned
   recovery aggregate. Lower-priority ambient items move to overflow
   before recovery text is shortened below its meaningful label.
3. Active-context truth keeps a stable cluster. If space is short, the
   label may condense, but the command-backed explanation remains one
   activation away.
4. Ongoing work aggregates by owning subsystem before it adds multiple
   visible items. Ten jobs become one inspectable work summary, not ten
   flickering counters.
5. Ambient metadata is useful but expendable. It is the first class
   moved into the status menu.

## Stable slots

The status bar uses stable slot keys so high-frequency state does not
move every time a provider warms, a counter ticks, or an extension
activates.

| Stable slot | Admits | Expectation |
| --- | --- | --- |
| `status.slot.recovery.primary` | `recovery_critical` | Always visible at supported widths; opens recovery or state-specific inspector. |
| `status.slot.context.workspace` | `active_context_truth`, shell-owned recovery summary | Workspace trust, profile, host, route, and workspace status projection. |
| `status.slot.context.execution` | `active_context_truth` | Current target, interpreter, SDK, debug/run posture. |
| `status.slot.context.vcs` | `active_context_truth` | Branch, repo state, source-control health, or mixed-root summary. |
| `status.slot.work.summary` | `ongoing_work` | Aggregate for indexing, tests, sync, AI, downloads, and long-running jobs. |
| `status.slot.efficiency.state` | first-party `ongoing_work` | Power, battery, thermal, or policy pressure that changed runtime behavior; opens the efficiency-state detail surface. |
| `status.slot.metadata.editor` | `ambient_metadata` | Cursor, selection, indentation, language mode. |
| `status.slot.metadata.file` | `ambient_metadata`, source-fidelity active context | Encoding, line endings, BOM, final newline, read-only/generated cues. |
| `status.slot.extension.scoped` | extension `ongoing_work` or `ambient_metadata` | Budgeted extension state only; never recovery-critical by direct extension request. |
| `status.slot.overflow.summary` | overflow trigger | Names hidden count and opens the status menu. |

High-frequency items such as branch, target, encoding, trust mode, cursor
position, and work counters must declare a stable slot expectation in
their status item record. They may update their value, but they must not
cause full-bar reflow.

## Compact overflow

The status bar is required on every supported adaptive class. Compact
layouts narrow detail; they do not make status truth disappear.

Overflow order at compact width:

1. `ambient_metadata`
2. extension-contributed `ongoing_work`
3. first-party `ongoing_work`
4. `active_context_truth` only after condensing and only when the same
   label and explanation remain in the status menu
5. `recovery_critical` never overflows at supported widths unless a
   shell-owned recovery aggregate remains visible and opens the complete
   recovery list

The overflow trigger must name the hidden count, such as `3 more status
items`. An empty unlabeled menu trigger is non-conforming.

Every overflowed item must preserve:

- the same label used when visible;
- the same short explanation of the current value;
- the same primary command id or a command that focuses the same owner
  surface;
- command palette and keyboard-search discoverability using the same
  label and current-value terms;
- focus return to the invoking status bar, menu, palette row, or owner
  surface.

Items hidden because they are not applicable do not need an overflow row.
Items suppressed by budget, policy, or extension review need a discoverable
explanation in the owning extension, policy, or support surface.

## Action contract

Each status item must route to the narrowest useful explanation or action.

| Item kind | Primary activation should open | Non-conforming activation |
| --- | --- | --- |
| Recovery-critical state | Specific recovery inspector, failure sheet, status drawer, diff/review surface, or repair command. | Generic settings, generic help home, or a broad dashboard when a narrow inspector exists. |
| Active context truth | Execution-context inspector, branch/source-control detail, trust review, host/route detail, profile detail. | A generic preferences page that does not explain the current value. |
| Ongoing work | Owning durable job row, task surface, activity row, or aggregate work menu. | Toast history or a blank panel with no current job selected. |
| Ambient metadata | Specific command or inspector for that metadata, such as encoding, indentation, language mode, or cursor/selection detail. | A settings detour without showing the current value first. |

Settings may appear as a secondary action after the current value is
explained. The primary action must not be settings-only.

Every status item record must carry:

- `label`;
- `explanation`;
- `current_value_label`;
- `primary_command_id`;
- `primary_route_id = route.status_bar_link` unless the item is only
  represented in palette or keyboard search;
- `opens_surface_ref` or `explanation_target_ref`;
- parity fields for visible, status-menu, palette, and keyboard-search
  rendering.

## Extension contribution budget

Extensions may contribute status items only through the status item
schema. They do not get private chrome, private priority bands, or direct
access to recovery-critical slots.

Default limits:

| Budget | Normal width | Compact width |
| --- | ---: | ---: |
| Visible items per extension | 1 | 0, unless the item is active ongoing work |
| Visible extension items across the whole bar | 2 | 1 |
| Extension items that may claim `status.slot.extension.scoped` | 1 per extension | 1 aggregate across extensions |

Extension rules:

1. Extension-contributed items default to `ambient_metadata` or
   `ongoing_work`.
2. An extension cannot directly contribute `recovery_critical`. It may
   emit a lifecycle, activity, or failure record; the shell may synthesize
   a shell-owned recovery item from that record.
3. Branding-only, account-nag, promotional, upgrade, marketplace,
   release-note, and vanity-count items are denied.
4. A contributed item must have a primary command that explains the
   current value in the extension's detail surface, task row, or
   performance/permission inspector.
5. Extension items are displaced before first-party active context and
   before any recovery-critical state.
6. Extension work counters must aggregate. A status bar cannot become a
   per-extension activity ticker.

When the extension budget is exceeded, lower-priority extension items move
to the status menu or extension surface with the same label and explanation.
If the request is branding-only, the item is denied rather than overflowed.

## Anti-jitter and animation

Status item updates must not reflow the full bar.

Rules:

1. Spinners render inside a reserved glyph box. The spinner frame cannot
   change item width.
2. Numeric counters use a reserved width for the expected maximum label,
   a monospace counter segment, or an aggregate label that does not change
   width on every tick.
3. Rapid updates coalesce to the owning item or work aggregate. They do
   not trigger sibling layout measurement on every event.
4. Items that change more often than once per second must declare
   `high_frequency = true` and a fixed or reserved width strategy.
5. When a severe item appears, layout may displace lower-priority items,
   but the recovery slot remains stable after the transition. Continuous
   shifting while the severe state persists is non-conforming.

## Required examples

The fixture corpus covers these acceptance-critical cases:

- canonical status item catalog and priority ladder;
- save failure as a recovery-critical item with stable placement and a
  narrow inspector;
- extension-contributed ongoing work inside the contribution budget;
- compact crowded status bar where ambient and extension items overflow
  while severe state stays visible;
- denied extension branding/vanity request.
