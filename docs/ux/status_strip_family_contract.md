# Status-strip and readiness-banner family contract

This document freezes the shared field family for top-of-surface status
strips and readiness banners. Workspace, environment, provider, graph,
and framework surfaces may render different layouts, but they read one
contract for scope, readiness, counts, hidden-scope disclosure, primary
next action, open-details routing, export, and screenshot-safe labels.

The goal is to stop each surface from inventing local synonyms for
"partially ready", "cached", "stale", "provider-backed", or "blocked".
When a reviewer compares a workspace strip, a provider strip, and a
graph strip, the state words and field names must line up.

## Companion artifacts

- [`/schemas/ux/status_strip.schema.json`](../../schemas/ux/status_strip.schema.json)
  defines the machine-readable `status_strip_record`.
- [`/fixtures/ux/status_strips/`](../../fixtures/ux/status_strips/)
  contains worked examples for workspace, environment, provider, graph,
  and framework strips.

## Upstream contracts

This contract composes with existing owners and does not replace them:

- [`degraded_mode_pattern.md`](./degraded_mode_pattern.md) for lifecycle
  status labels and safe recovery actions.
- [`transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  for transport posture, environment status cells, and repair cards.
- [`status_bar_contract.md`](./status_bar_contract.md) for ambient status
  item priority and overflow behavior.
- [`search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  for search and graph readiness states.
- [`provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md)
  for provider health, freshness, locality, scope claim, and result
  provenance.
- [`project_graph_and_indexing_seed.md`](../graph/project_graph_and_indexing_seed.md)
  for graph epoch, hot-set, completeness, and readiness export refs.

If this document conflicts with any upstream contract above, the
upstream owner wins and this document plus the schema and fixtures must
change in the same patch.

## Scope

Frozen at this revision:

- one `status_strip_record` for top-of-surface status strips and
  readiness banners;
- one seven-value `readiness_state` vocabulary shared by every strip;
- required shared anatomy for current scope or object, readiness,
  freshness, data backing, blocker and warning counts, hidden-scope
  notes, primary next action, and open-details path;
- allowed specialized variant blocks for workspace, environment,
  provider, graph, and framework surfaces;
- screenshot-safe and export-safe rules that preserve live, cached,
  provider-backed, partial, stale, degraded, and blocked truth.

Out of scope:

- final pixel layout, typography, iconography, or animation;
- implementation of every future strip-bearing surface;
- replacing deeper owner records such as environment status cells,
  provider status rows, graph readiness exports, or lifecycle cards.

## Shared Anatomy

Every strip carries the same top-level fields. A surface may hide or
condense fields visually, but it may not remove them from the backing
record or bury hidden/partial/stale truth in a secondary inspector only.

| Field | Required content | Non-conforming collapse |
| --- | --- | --- |
| `surface_family` | `workspace`, `environment`, `provider`, `graph`, or `framework`. | A custom family such as "smart status" with no mapped owner. |
| `presentation_class` | Whether this is a top status strip, readiness banner, workflow summary, compact strip, or support-export summary. | Treating an export summary as a screenshot-only rendering. |
| `current_scope` | Opaque scope or object ref, scope kind, privacy-safe label, summary sentence, and source record refs. | Relying on visible title text or raw path/provider labels as identity. |
| `readiness` | Shared readiness state, freshness class, data-backing mode, live-claim class, and disclosure booleans. | Saying "OK" while data is cached, stale, provider-backed, or partial. |
| `counts` | Typed blocker, warning, hidden-scope, partial, stale, and count-accuracy fields. | Naked numbers or warning text with no scope or cause. |
| `primary_next_action` | One command-backed next action, safety sentence, and target ref. | Generic retry or settings detour when a narrow detail path exists. |
| `open_details_path` | Keyboard-reachable details route, focus target, accessible name, and export fields. | Details available only by hover, screenshots, raw logs, or support-only dumps. |
| `variant` | Exactly one surface-specific extension block matching `surface_family`. | Surface-local extra fields that rename shared state. |
| `export_and_screenshot_safety` | Redaction, export fields, screenshot cues, and invariants preserving state vocabulary. | Exporting prose that loses whether the strip was live, cached, provider-backed, partial, stale, degraded, or blocked. |
| `accessibility` | Announcement, keyboard reachability, and focus-return target. | Color-only state or pointer-only inspection. |

## Shared State Vocabulary

Every strip resolves to exactly one `readiness_state` value. Variant
blocks may carry deeper owner state, but their top-level strip state
must map to this set.

| `readiness_state` | Meaning | Required disclosure |
| --- | --- | --- |
| `loading` | The owner has started and no stable result is ready for the current scope yet. | Name what is loading and where details open. |
| `warm` | Enough state is ready for useful interaction, but the strip is not claiming full current coverage. | Name the warm or cached basis. |
| `partial` | Some in-scope data, lanes, roots, providers, or framework facts are missing, deferred, hidden, or unavailable. | `partial_scope_note` is required and visible at top level. |
| `stale` | A prior value is shown past its freshness floor or after an invalidating event. | `stale_note` is required and freshness must be `stale`. |
| `degraded` | The surface remains usable but a provider, fallback, authority, transport, or certainty path is reduced. | At least one warning or blocker count is required. |
| `blocked` | The primary workflow cannot continue for the current scope until a blocker is resolved or the scope narrows. | `blocker_count` must be greater than zero and the live claim is denied. |
| `ready` | Fresh-enough evidence covers the declared scope and no top-level blocker, warning, hidden, partial, or stale cue applies. | No hidden, partial, stale, warning, or blocker counts may remain. |

Allowed deeper owner vocabularies still travel in the variant block:
environment cells can say `degraded_resolved`; provider rows can say
`cached_only`; graph shards can say `hot_set_ready`; framework packs can
say `mixed_exact_and_heuristic`. The shared strip state is the reviewer
comparison axis.

## Data-Backing and Freshness Rules

`readiness.backing_mode` tells whether the view is live, cached,
provider-backed, local-only, imported, or mixed. It is orthogonal to
`readiness_state`:

- `live` may render as `ready`, `warm`, `degraded`, or `blocked`
  depending on scope and blockers.
- `cached`, `provider_backed_cached`, `provider_backed_stale`, and
  `mixed_live_and_cached` must render a freshness cue on the strip and
  in exports.
- `provider_backed_live`, `provider_backed_cached`, and
  `provider_backed_stale` must set `provider_backing_disclosed = true`
  and include the `provider_backed_label` screenshot cue.
- `partial` state must set `partial_readiness_disclosed = true` and
  include the `partial_readiness_label` screenshot cue.
- `stale` state must set `stale_data_disclosed = true`, set
  `freshness_class = stale`, and include the `stale_label` screenshot
  cue.

Generic copy such as "healthy", "up to date", "connected", or "smart"
is non-conforming when the backing mode is not live and complete for the
declared scope.

## Count Rules

Counts are part of the strip, not a hidden detail:

- `blocker_count` counts workflow-blocking issues for the current scope.
- `warning_count` counts non-blocking but material degraded, mismatch,
  freshness, compatibility, or support issues.
- `hidden_scope_count` counts known rows, roots, lanes, providers, or
  facts withheld by policy, filters, workset scope, disconnected remote
  shards, provider authorization, redaction, or unloaded scope.
- `hidden_scope_note` is required when `hidden_scope_count > 0`.
- `partial_scope_note` is required for `readiness_state = partial`.
- `stale_note` is required for `readiness_state = stale`.

Counts may be exact, approximate, or upper-bound. Approximate and
upper-bound counts must say so through `count_accuracy` and may not be
rendered as exact integers in screenshots or exports.

## Variant Extensions

Each strip carries exactly one variant block matching `surface_family`.
Variant blocks extend the shared anatomy; they do not rename or override
it.

### Workspace Variant

The workspace variant names:

- workspace scope class: single root, multi-root, active workset, sparse
  slice, managed workspace, or restricted workspace;
- workspace status refs such as lifecycle cards, title-context rows, or
  status items;
- workspace truth refs for trust, restore, workset, root, save journal,
  or bootstrap state;
- root counts for included, excluded, hidden, and blocked roots;
- whether local editing and local continuation remain available.

Workspace strips must not imply that local editing is blocked merely
because index, graph, remote, provider, or framework readiness is below
ideal. If local editing is blocked, the strip must say why.

### Environment Variant

The environment variant links to:

- the `environment_status_strip_record`;
- the execution-context snapshot;
- active cell kinds for interpreter/toolchain, SDK, shell, and target;
- transport posture;
- status cell refs and target refs.

The variant owns environment-specific details while the shared
`readiness_state` communicates the top-level state. For example, a
target cell may be `stale_unverified`, while the family strip resolves
to `stale`.

### Provider Variant

The provider variant names:

- provider surface class: language intelligence, code host, issue or
  planning, CI/checks, docs or portal, artifact registry, AI provider,
  or managed admin;
- provider family refs and provider-status row refs;
- locality classes;
- actor or authority refs where relevant;
- a provider-backing sentence.

Provider-backed strips must keep provider backing visible. Imported or
cached provider state must never render as locally authored or live
local truth.

### Graph Variant

The graph variant names:

- graph epoch ref;
- workset or slice ref;
- search/graph readiness state such as `hot_set_ready`, `partial_index`,
  `warm_index`, `fully_indexed`, or `stale_index`;
- missing lane count and lane refs;
- readiness export refs and project node refs.

Graph strips must keep hot-set, partial, stale, imported, policy-hidden,
and missing-lane truth visible before graph-backed actions. A graph
surface cannot silently downgrade to lexical or parser-only reasoning
without updating the strip state and detail path.

### Framework Variant

The framework variant names:

- framework pack ref;
- framework label and version label;
- support class;
- certainty class;
- compatibility note;
- route, component, generator, or overlay refs.

Framework strips must separate exact framework facts from imported,
heuristic, stale, or blocked framework facts. A framework pack may be
usable while only part of a route graph or generated overlay is ready;
that state resolves to `partial`, `stale`, or `degraded` rather than a
surface-local synonym.

## Export and Screenshot Safety

Status strips often appear in support screenshots and exported workflow
summaries. The backing record therefore carries two sets of rules.

Screenshot-safe rendering must include:

- scope label;
- state label from the seven-value vocabulary;
- freshness label when freshness is not authoritative live;
- hidden-scope note when `hidden_scope_count > 0`;
- provider-backed label when backing mode is provider-backed;
- partial readiness label when `readiness_state = partial`;
- stale label when `readiness_state = stale`;
- detail affordance label.

Export-safe rendering must preserve:

- `surface_family`, `current_scope`, `readiness_state`,
  `freshness_class`, `backing_mode`, and `live_claim_class`;
- blocker, warning, hidden, partial, and stale count fields;
- primary action id and detail route;
- variant owner refs;
- redaction class and raw-private-material exclusion.

Raw paths, raw URLs, raw host names, raw provider payloads, raw user
identifiers, raw logs, raw prompts, raw command bodies, raw tokens, raw
secret values, and raw policy bodies never cross this boundary.

## Details and Accessibility

The open-details path is mandatory because the strip is a summary, not
a full inspector. It must name:

- the detail surface class;
- the command id;
- the focus target;
- an accessible name;
- whether it opens without pointer input;
- export fields that preserve the same facts.

The strip must announce the state and scope to assistive technology. It
must not rely on color, animation, hover-only text, or screenshots to
communicate blocked, partial, hidden, stale, provider-backed, or cached
state.

## Fixture Requirements

Fixtures under `/fixtures/ux/status_strips/` must:

- validate as `status_strip_record`;
- cover every `surface_family`;
- exercise at least one `partial`, one `stale`, one `degraded`, one
  `blocked`, and one `ready` or `warm` state across the corpus;
- show hidden scope, partial readiness, and stale data on top-level
  fields;
- preserve provider-backed and cached data in screenshot and export
  cues;
- link to the deeper owner records they summarize.

## Acceptance Checklist

A status strip or readiness banner is conforming when:

1. It uses exactly one of the seven shared `readiness_state` values.
2. Current scope or object identity is explicit and exportable by opaque
   ref.
3. Blocker, warning, hidden, partial, and stale facts are top-level
   fields, not secondary-inspector-only prose.
4. The primary next action is command-backed and names what it will
   open or change.
5. The open-details path is keyboard reachable and preserves focus
   return.
6. The variant block matches the surface family and references deeper
   owner records instead of copying or renaming them.
7. Screenshot and export cues preserve live, cached, provider-backed,
   partial, stale, degraded, and blocked truth without widening
   redaction.
