# Desktop affordance contract: OS entry, deep-link intent, and lifecycle recovery

This document is the cross-surface contract for native desktop
affordances in Aureline. It exists so system open, file associations,
open-with, reveal-in-system-shell, native dialogs, dock / taskbar
presence, OS notifications, badges, system share targets, copied paths
or permalinks, drag-drop, open-from-terminal, and default-browser
handoffs all route through one product truth model instead of becoming
per-platform side effects.

The contract is normative. Where this document disagrees with the PRD,
TAD, TDD, or UI / UX spec, those sources win and this document plus its
companion artifacts MUST update in the same change. Where this document
disagrees with a surface-local handler, platform adapter, installer, or
notification implementation, this document wins and the adapter is
non-conforming.

The companion artifacts are:

- [`/schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
  - boundary schema for `deep_link_intent_record` and
  `system_affordance_case_record`. The schema freezes route class,
  source surface, target identity, command-id linkage, trust / policy
  review, replay posture, degraded fallback, handler ownership, privacy
  posture, and audit outcomes.
- [`/fixtures/platform/system_affordance_cases/`](../../fixtures/platform/system_affordance_cases/)
  - worked fixtures covering file association entry, notification
  click-through, badge lineage, lock-screen privacy, removable-volume
  return, deep-link denial, native dialog routing, system share / copy
  affordances, and open-from-terminal continuity.

This contract rides alongside and does not re-mint the vocabularies
already frozen in:

- [`/docs/platform/desktop_platform_conformance_matrix.md`](../platform/desktop_platform_conformance_matrix.md)
  - claimed desktop profiles, handler ownership claims, notification /
  badge rows, native dialog routing, display topology, and explicitly
  unclaimed lanes.
- [`/docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md)
  and
  [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  - canonical event id, notification route, badge class, redaction,
  click-through linkback, and dismissal / acknowledge / resolve
  semantics.
- [`/docs/ux/clipboard_history_contract.md`](./clipboard_history_contract.md)
  - copy variants, drag-drop result verbs, suspicious-content handling,
  undo lineage, and large-transfer feedback.
- [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  - stable command IDs, descriptor scope, enablement decisions,
  preview / approval posture, invocation packets, and command
  lifecycle.
- [`/schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json)
  - open, clone, import, add-root, restore, recent-work, unavailable
  target, and source-surface vocabulary.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  - typed system-browser launch packets, provider destination class,
  replay posture, approval linkage, and return-anchor rules.
- [`/docs/qa/multi_window_verification.md`](../qa/multi_window_verification.md)
  and
  [`/fixtures/platform/window_display_cases/`](../../fixtures/platform/window_display_cases/)
  - display topology, mixed-DPI, wake / resume, off-screen recovery,
  and restart / reopen continuity drills.
- [`/docs/ux/window_display_contract.md`](./window_display_contract.md)
  and
  [`/schemas/platform/window_state.schema.json`](../../schemas/platform/window_state.schema.json)
  - native titlebar/control projection, fullscreen and snapped or tiled
  placement, virtual-desktop fallback, owned-prompt recentering, focus
  return, presentation-mode fallback, and restore-history vocabulary.

## Who reads this document

- Shell, installer, platform-adapter, notification, and windowing
  authors wiring native entry points and OS surfaces.
- Workspace, VFS, review, collaboration, auth, policy, command,
  clipboard, terminal, and browser-handoff owners whose product objects
  may be opened from outside the app shell.
- Support, parity-audit, release, accessibility, and conformance
  tooling that needs one inspectable trail for a system affordance.

## 1. Scope

This contract freezes:

- the invariant that every OS-facing affordance resolves to a canonical
  command ID, canonical event lineage, or canonical object identity
  before it can affect product state;
- the required disclosure for file associations, open-with,
  reveal-in-system-shell, native open / save dialogs, dock / taskbar /
  jump-list entries, OS notification click-through, badge counts, system
  share targets, copied paths or permalinks, drag-drop, and
  open-from-terminal;
- the deep-link intent envelope: route class, origin, target identity,
  command ID, trust / policy review, replay policy, degraded fallback,
  handler ownership, privacy posture, and audit outcome;
- lifecycle recovery behavior for wake-from-sleep, display reconnect,
  DPI / topology change, removable-volume return, expired sessions,
  blocked links, unavailable targets, and lock-screen privacy;
- default-browser handoff expectations for auth, provider console,
  docs, and review flows.

## 2. Out of Scope

- OS-specific implementation details for Launch Services, registered
  URI handlers, Windows toast APIs, Linux desktop files, notification
  daemons, portals, shell badges, or native dialog adapters. Claimed
  profile details live in the desktop-platform conformance matrix.
- Final user-facing microcopy. This contract pins fields, states,
  fallback classes, and evidence hooks; product writing and localization
  choose final strings inside those limits.
- A complete command-registry seed for every future platform command.
  The contract requires command-id linkage; registry rows may land in
  separate command-system work.

## 3. Non-Negotiable Invariants

1. **No opaque OS side effects.** Every affordance must name at least
   one of: `command_id_ref`, `canonical_event_id_ref`,
   `event_lineage_ref`, or `object_identity_ref`. A handler that only
   says "open this URL" or "show this path" is not enough.
2. **No authority widening from the OS.** An OS entry path may never
   grant more trust, policy authority, remote authority, collaboration
   presence, or target certainty than the in-product object carries.
3. **Review before boundary change.** If the intent crosses a trust,
   policy, auth, remote, tenant, collaboration, external visibility, or
   destructive boundary, Aureline must show an in-product review surface
   before execution.
4. **Replay is denied by default.** Deep-link and browser-return tokens
   are single-use unless the intent record explicitly declares bounded
   reuse or read-only resumability. Consumed, expired, drifted, or
   policy-epoch-mismatched intents route to a denial or recovery sheet.
5. **Fallback preserves intent.** Missing, moved, blocked, or
   unavailable targets become placeholders, cached context, locate /
   reconnect actions, or denial rows that preserve the original source
   and target identity. Opening a generic empty shell is not an
   acceptable fallback.
6. **Lock-screen privacy wins.** OS notifications, lock-screen
   summaries, shell recents, badges, and companion previews carry the
   narrowest privacy-safe payload unless user or admin policy explicitly
   permits more disclosure.
7. **Handler ownership is visible.** Side-by-side channels, portable
   installs, managed installs, and updates must preview handler, recent
   item, file association, protocol, notification, and badge ownership
   changes. Last-writer-wins ownership is not claim-bearing behavior.
8. **Keyboard and accessibility parity remains intact.** Any action
   reachable from an OS affordance must have a product command path,
   focus-return rule, and accessible recovery state.

## 4. Affordance Matrix

| Affordance | Required anchors | Required disclosure | Forbidden behavior |
|---|---|---|---|
| System open / file association | command ID plus target object identity | literal user target, resulting mode, handler owner, active profile or channel, trust / policy implication, recovery path | guessing file vs folder vs workspace, bypassing trust review, or silently seizing handler ownership |
| Open-with / reveal-in-system-shell | command ID plus canonical target identity | canonical vs presentation path posture, unavailable / remote / generated boundary, read-only or reveal-only mode | rewriting a target, following an alias before risky write disclosure, or revealing a stale path as current truth |
| Native open / save dialog | command ID plus VFS or save-target token | overwrite risk, read-only state, remote / generated / removable boundary, checkpoint or recovery option | platform dialog copy that hides policy, alias, or write-safety state |
| Dock / taskbar / jump-list / recent entry | object identity plus handler owner | channel / build owner, stale or unavailable marker, safe reopen mode | privileged or mutating action from a summary-only shell surface |
| OS notification click-through | canonical event id plus linkback target | event class, privacy mode, exact object or placeholder, bounded action | opening a generic home screen, losing source object, or applying a high-risk action directly |
| Badge / presence indicator | canonical event lineage or durable object set | count class, suppression state, quiet-hours state, source object class | mixed-class badge inflation or a badge persisting after durable state clears |
| System share target | command ID plus share object identity | source object, share audience / visibility class, redaction and export posture, policy review need | sharing raw local paths, raw URLs, secrets, or broader scope than the source object admits |
| Copy path / copy permalink | command ID plus representation class | literal vs canonical vs relative path, permalink route scope, expiry / replay state, redaction class | presenting a copied value as globally valid when it depends on local trust, policy, or current workspace |
| Default-browser handoff | browser-handoff packet plus return anchor | destination class, reason, expected authority, privacy consequence, replay expiry, local fallback | launching raw URLs from protected surfaces or returning through an unvalidated origin |
| Drag-drop / open-from-terminal | command ID plus source surface and target identity | verb, target kind, resulting mode, trust / policy state, focus-return target | hidden writes, unreviewed package / extension import, or terminal cwd inference that outruns target identity |

## 5. Deep-Link Intent Lifecycle

A deep-link intent is not a command. It is an externally-originated
request to resolve a product command and target under current trust,
policy, target availability, and replay conditions.

Every deep-link intent follows this sequence:

1. **Capture the raw entry boundary.** The platform adapter captures
   source surface, handler owner, origin class, opaque origin ref, and
   arrival time. Raw URLs, callback bodies, provider payloads, and raw
   local paths do not cross the boundary.
2. **Classify the route.** The adapter resolves the route class:
   local file, workspace, review / work item, auth callback,
   collaboration join, managed workspace resume, command invocation,
   external-browser return, settings / policy review, support /
   incident, provider-console handoff, or unavailable-target recovery.
3. **Bind target identity.** The resolver binds a canonical target
   object or an unavailable-target placeholder. Target identity includes
   object kind, opaque object ref, optional revision / freshness, and
   current availability.
4. **Resolve the command descriptor.** The resolver maps the route to a
   canonical `command_id_ref`, then evaluates the command descriptor,
   enablement decision, preview class, approval posture, and client
   scope. Deep links may not mint commands.
5. **Evaluate trust and policy.** The resolver compares requested action
   class, authority delta, current trust state, policy epoch, tenant /
   workspace scope, handler owner, and target freshness.
6. **Evaluate replay.** Single-use intents must not run after
   consumption. Expired, drifted, target-changed, origin-changed, or
   policy-epoch-changed intents deny with a replay-deny posture and
   route to recovery if one exists.
7. **Show review when needed.** Boundary-raising intents land on an
   interstitial or review sheet naming origin, action class, target
   identity, command ID, boundary change, replay posture, and fallback.
8. **Execute or degrade.** Only admitted intents invoke the command.
   Denied or degraded intents preserve source and target as a review
   row, placeholder, cached context, locate / reconnect / reauth action,
   or explicit denial.

### Route Classes

| Route class | Allowed primary outcome | Review trigger |
|---|---|---|
| `local_file_open` | open file or workspace candidate | target outside current trust scope, read-only / generated target, unknown file association |
| `workspace_open` | open workspace, add folder, or restore recent context | profile, channel, trust, policy, or handler ownership changes |
| `review_or_work_item` | open anchored review / work item context | provider auth missing, local clone needed, target drift, write-capable action |
| `auth_callback` | complete sign-in or step-up | tenant mismatch, origin mismatch, replay reuse, scope widening |
| `collaboration_session_join` | join or inspect collaboration session | role, retention, external guest, presenter / driver, or shared-control boundary |
| `managed_workspace_resume` | resume, rebuild review, or open cached context | expired session, region / template drift, auth or policy freshness required |
| `command_invocation` | reviewed command invocation | any mutating, privileged, remote, or policy-bearing command |
| `external_browser_return` | return to object or fallback context | origin mismatch, stale return anchor, policy epoch drift |
| `settings_or_policy_review` | open exact settings / policy surface | policy locked, admin-only, or unavailable client scope |
| `support_or_incident` | open incident, support packet, or export review | redaction, retention, identity, or external issue route change |
| `provider_console_handoff` | open handoff packet or browser fallback | in-product parity unavailable, provider authority differs |
| `unavailable_target_recovery` | placeholder, locate, reconnect, cached context, or denial | always discloses why exact open is unavailable |

## 6. Review and Denial Rules

The following conditions require review before any command executes:

- authority delta is anything other than `none`;
- origin class is unknown, untrusted, or lower trust than the requested
  route;
- target identity is missing, moved, ambiguous, stale, unavailable, or
  outside loaded scope;
- current trust state, policy epoch, tenant scope, profile, or channel
  differs from the minted intent;
- command descriptor requires preview, approval, step-up, or policy
  review in the current context;
- replay posture is expired, consumed, policy-epoch changed, target
  drifted, or otherwise denied;
- handler ownership conflicts or would change the owning build /
  channel;
- the action would join collaboration presence, reopen remote context,
  resume managed state, publish externally, or mutate provider state.

Blocked intents MUST emit a denial reason that tooling can compare. A
blocked deep link may offer `Open cached context`, `Locate target`,
`Reconnect`, `Reauthenticate`, `Open in default browser`, `Export
context`, or `Dismiss`, but it may not silently downgrade to a broader
or unrelated command.

## 7. Desktop Lifecycle Failure Contracts

| State | Required behavior | Must never happen |
|---|---|---|
| Wake from sleep | mark remote, auth, callback, route, and collaboration surfaces reconnecting or stale until revalidated; preserve local edit context | silent rerun, hidden focus steal, privileged session reuse, or remote write replay |
| Display reconnect | move windows / sheets into reachable bounds, preserve focus chain where possible, show recenter or review-layout action | dialog stranded off-screen or focus left on a disconnected display |
| DPI / topology change | reflow readable shell, keep command surfaces reachable, preserve pane and window roles | scale change hiding controls, overlapping recovery actions, or losing sheet ownership |
| Removable-volume return | match the returning root to last-seen identity, disclose stale / changed / exact, then offer reopen or reconcile | silently retargeting to a different mount or discarding recovered buffers |
| Expired session | show what paused, what remains local-only, and reauth / reconnect / cached-context options | pretending managed, remote, or provider authority is still live |
| Blocked deep link | show origin, route, target identity if safe, denial reason, and bounded recovery | generic error page or silent empty workspace |
| Unavailable target | preserve original intent as a placeholder with locate / cached / remove / export options | destructive cleanup, hidden recent removal, or target guessing |
| Lock-screen privacy | deliver generic or count-only payloads unless disclosure is explicitly allowed | raw code, raw paths, tenant names, prompt text, secrets, or high-risk details on lock screen |

Lifecycle recovery is context preservation, not automatic execution.
The product may attempt reconnection, target lookup, or display
recovery, but it may not reapply writes, rejoin privileged sessions, or
rerun commands without fresh user intent.

## 8. Notifications, Badges, and Collaboration Presence

OS notifications and badges are projections of the durable attention
model. They do not create private state.

- Every OS notification click-through cites a canonical event id and a
  durable linkback target. If the target no longer exists, the click
  opens a truthful placeholder with source, freshness, and recovery
  actions.
- Notification actions are bounded to inspect, open, acknowledge,
  snooze, retry attach, or open handoff unless an in-product review
  surface commits the consequence-bearing command.
- Badge counts derive from deduped durable objects grouped by count
  class, such as review requests, failed runs, mentions, paused sync, or
  blocked trust changes. Mixed-class raw event counts are forbidden.
- Quiet hours, focus mode, presentation mode, screen sharing, and admin
  suppression apply consistently to in-product toasts, OS
  notifications, badges, and companion handoff alerts.
- Collaboration presence notifications must state the session object,
  role / presence class, freshness, and authority limit. A stale,
  disconnected, or read-only companion / browser state may not imply
  live desktop participation, shared-control authority, or presenter
  control.
- Lock-screen collaboration payloads degrade to generic class and count
  unless policy explicitly allows object identity.

## 9. Native Dialogs, Browser Handoff, and Terminal Entry

### Native Open and Save Dialogs

Native open / save dialogs MAY use platform controls, but their product
meaning is fixed:

- open dialogs resolve to the same entry verbs and resulting modes used
  by Start Center, command palette, drag-drop, system open, and CLI;
- save dialogs carry save-target token, write posture, alias /
  canonical-path truth, read-only state, overwrite risk, checkpoint /
  recovery posture, and remote / generated / removable boundary;
- dialog dismissal, unavailable target, or policy block returns focus to
  the invoking product surface with the original intent preserved.

### Default-Browser Handoff

Any protected surface that leaves Aureline for the system browser emits
a `browser_handoff_packet_record` or equivalent object identity. The
handoff names destination class, reason, object identity, expected
authority, privacy consequence, return anchor, replay expiry, and local
fallback. Auth returns and provider callbacks re-enter as
`deep_link_intent_record` rows and must pass origin, action, scope,
policy, and replay validation before authority changes.

### Clipboard, Drag-Drop, Share, and Terminal

- Copy path and copy permalink variants must disclose whether the value
  is literal, canonical, relative to the current workspace, or a
  route-token / permalink that must be re-resolved later.
- System share targets emit an explicit share intent with redaction,
  audience, expiry, policy, and source-object identity. Sharing never
  widens data access beyond the source object.
- Drag-drop entry uses the clipboard / drag-drop result verbs and the
  same target identity and trust review as system open.
- Open-from-terminal preserves cwd, argv intent, source process class,
  profile, and trust review. Terminal-originated open may infer a
  candidate root, but the user-facing entry row must distinguish the
  literal terminal request from the canonical target Aureline resolves.

## 10. Fixture Rules

Every system-affordance fixture under
[`/fixtures/platform/system_affordance_cases/`](../../fixtures/platform/system_affordance_cases/)
must:

- name one case class and one source surface class from the schema;
- include canonical backing refs for command id, object identity, and
  event lineage where applicable;
- declare whether review is required, why, and which fallback preserves
  intent when exact execution is denied;
- distinguish in-product payload, OS notification payload, lock-screen
  payload, and exported / support payload privacy where applicable;
- avoid raw paths, raw URLs, raw callback bodies, raw provider payloads,
  raw credentials, raw prompt text, and raw customer-owned identifiers;
- list at least one `must_not_happen` assertion tied to authority,
  target certainty, privacy, replay, or lifecycle recovery.

## 11. Review Checklist

A change touching a desktop affordance is conforming only if reviewers
can answer:

1. Which canonical command, event lineage, or object identity backs the
   OS-facing action?
2. What trust, policy, tenant, profile, handler-owner, or replay
   boundary changed, and where is it disclosed before execution?
3. What happens when the target is missing, blocked, stale,
   unavailable, or privacy-sensitive?
4. Does a notification, badge, share target, copied permalink, or
   shell recent imply more authority, presence, or certainty than the
   source object carries?
5. Does wake, display reconnect, DPI / topology change, removable-volume
   return, or expired session preserve context without destructive
   cleanup, hidden focus stealing, or silent rerun?
