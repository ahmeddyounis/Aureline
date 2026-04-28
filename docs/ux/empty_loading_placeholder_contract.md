# Empty, Loading, Skeleton, and Placeholder-Honesty Contract

This document freezes the cross-surface truth model for empty, loading,
indexing, probing, blocked, degraded, missing-dependency, partial-data,
and stale-cached placeholder surfaces. It exists so shell, Start Center,
search, docs/help, install review, and provider-backed surfaces do not
invent local readiness language or imply that content is ready while the
owning scope is still warming, partial, blocked, stale, or unavailable.

Machine-readable companions:

- [`/schemas/ux/placeholder_state.schema.json`](../../schemas/ux/placeholder_state.schema.json)
  defines `placeholder_state_record`, the boundary record every
  placeholder-bearing surface can emit to product, QA, support export,
  screenshot review, and parity tooling.
- [`/fixtures/ux/placeholder_cases/`](../../fixtures/ux/placeholder_cases/)
  contains worked cases for no-results, first-run, indexing in progress,
  provider unavailable, and stale cached content.

This contract composes with and does not replace:

- [`state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md)
  for required empty-state axes, loading-state anti-patterns, failure
  tiers, and recovery placement.
- [`view_freshness_contract.md`](./view_freshness_contract.md) for
  live, snapshot, partial, stale, and approximate view disclosure.
- [`degraded_mode_pattern.md`](./degraded_mode_pattern.md) for preserved
  capability, reduced capability, last-failure, and safe recovery slots.
- [`status_strip_family_contract.md`](./status_strip_family_contract.md)
  for top-of-surface readiness summaries.
- [`../search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  for search-specific index states and frozen partial/stale sentences.
- [`start_center_contract.md`](./start_center_contract.md),
  [`../verification/docs_browser_packet.md`](../verification/docs_browser_packet.md),
  [`../verification/install_review_packet.md`](../verification/install_review_packet.md),
  and [`../providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  for surface-specific projection rules.

If this document conflicts with the source PRD, architecture, technical
design, UI/UX spec, or design-system guide, the source wins and this
document, schema, and fixtures update in the same change.

## Core Rule

A placeholder is allowed only when it tells the truth about readiness.
The placeholder may reserve space, keep prior content visible, or stream
partial data, but it must not simulate authoritative content or render a
green / ready state unless fresh-enough evidence covers the declared
scope and no partial, probing, blocked, degraded, missing, stale, cached,
or hidden-scope cue remains.

Every placeholder-bearing surface records four separate axes:

1. **State class** - what kind of non-final state the user is seeing.
2. **Content basis** - whether the surface is empty, skeleton-shaped,
   progressively hydrated, retained from cache, stale, or explicit text.
3. **Readiness cue** - the visible badge/copy and accessible name.
4. **Next action** - the safest command-backed action or explanation
   route.

Collapsing these axes into "Loading", "Unavailable", "No results", or
"Ready" is non-conforming when a more precise state is known.

## Canonical State Classes

| State class | Use when | Required cue | Required next action |
|---|---|---|---|
| `empty_no_results` | The owning scope finished the relevant query and the current content set is genuinely zero. | Area purpose, current scope/filter, and "no results" wording. | Clear filters, widen/narrow scope, create/import when relevant. |
| `empty_first_run` | No durable work exists yet for this profile, workspace, or route. | Area purpose, first-run cause, local path availability. | Open folder/workspace, clone/import, or continue locally. |
| `loading` | Content is being computed, fetched, or materialised and no stable subset is ready. | What is loading and what remains usable. | Cancel, details, or continue with available chrome. |
| `indexing` | Search, graph, docs, or semantic lanes are building while a useful subset may already exist. | Warming/partial chip, covered scope, omitted scope. | Open index details, continue with available results, widen later. |
| `probing` | A runtime, provider, extension, install target, or remote is being checked before readiness is known. | Probe owner, stage, and authority boundary. | Cancel/retry, inspect probe, continue with safe local subset. |
| `blocked` | Policy, trust, permission, credential, or authority denies the path before commit. | Blocking source and blocked capability. | Open policy/detail, request access, switch safe route, export evidence. |
| `degraded` | The surface remains usable through a reduced or fallback path. | What still works and what is reduced. | Repair, retry, continue limited, reveal last failure. |
| `missing_extension_or_provider` | A required extension, provider, registry, docs pack, toolchain, or adapter is absent or unreachable. | Missing owner and whether cached/local fallback exists. | Install/enable, retry provider, open offline/local fallback. |
| `partial_data` | Some in-scope data is present and some is omitted, hidden, still warming, or provider-limited. | Covered scope, omitted scope, and reason. | Finish indexing, widen/narrow scope, explain hidden rows, export with omissions. |
| `stale_cached_content` | Prior content remains visible after freshness or causal continuity was lost. | Stale/cached label, last-known-good basis, invalidation cause. | Refresh, requery, continue read-only/local, export as stale. |

## Required Surface Anatomy

Every `placeholder_state_record` carries these slots:

| Slot | Required content |
|---|---|
| `surface_family` | One of the closed surface families: shell, Start Center, search, docs/help, install review, provider-backed, tree/list, status strip, or generic. |
| `state_class` | One canonical state class from the table above. |
| `truth_classes` | The truth classes involved, preserving durable, runtime, indexed, session, and inferred truth separately. |
| `degraded_state_tokens` | Any shared degraded tokens that apply, such as `Warming`, `Cached`, `Partial`, `Stale`, `Offline`, or `PolicyBlocked`. Empty exact states may use an empty set. |
| `state_summary` | Purpose, cause, preserved capability, reduced capability, and readiness basis. |
| `content_policy` | Whether the surface uses explicit text, skeletons, progressive rows, retained prior content, or a placeholder row. |
| `prior_content` | Whether cached/stale/partial prior content can coexist with the current state and what label is required. |
| `visible_cues` | Primary visible label, cause label, readiness badge, accessible name, and details route label. |
| `next_actions` | At least one command-backed safe action or detail route. |
| `readiness_guards` | Denials for ready/green/success treatment, content simulation, tooltip-only state, and unsupported placeholder forms. |
| `cross_surface_projection` | How this record maps to status-strip, view-freshness, search-readiness, Start Center, docs/help, install-review, or provider-mode language. |
| `accessibility` | Focus target, announcement text, keyboard reachability, and reduced-motion equivalence. |
| `support_export` | Export fields that preserve state class, content basis, stale/cache label, next action, and redaction posture. |

## Skeleton, Shimmer, and Placeholder Rules

Skeletons and shimmers are visual loading affordances, not permission to
pretend real content exists.

1. Skeletons are allowed only when the final row anatomy is predictable.
   Skeleton columns, row height, leading icon space, and action slot
   must collapse into the arrived row without layout shift.
2. Skeletons must not carry fake titles, fake counts, fake avatars, fake
   package names, fake tests, fake provider names, fake paths, fake
   diagnostics, or fake success indicators.
3. Shimmer is optional and must stop when reduced-motion settings request
   a static fallback. The fallback is a static placeholder plus progress
   text, not a blank panel.
4. Placeholder rows that represent missing extensions, unreachable
   providers, or held notifications must name the missing owner or hold
   condition. A generic blank row is non-conforming.
5. Progressive hydration may stream rows as they arrive, but it must
   keep above-the-fold identity stable and mark partial result sets until
   the declared scope is complete.
6. When the final structure is unknown, use explicit text and a bounded
   action instead of skeletons. Simulating a row layout that later turns
   into a different object type is non-conforming.
7. If a placeholder would hide a blocker, policy denial, provider
   outage, stale cache, or partial scope, downgrade to explicit text.

## Cached and Stale Prior Content

Prior content may coexist with a placeholder only when the label makes
the basis obvious.

| Prior-content class | Allowed with | Required label | Forbidden claim |
|---|---|---|---|
| `no_prior_content` | First-run, true empty, first load. | None beyond empty/loading cue. | Prior work or current results. |
| `cached_prior_content_visible` | Offline/local fallback, provider unavailable, warm cache. | `Cached` plus source/age class. | Live or fresh provider state. |
| `stale_prior_content_visible` | Cache past freshness floor, invalidated index, drifted provider snapshot. | `Stale` plus last-known-good basis and refresh path. | Ready, passing, current, exact, or green. |
| `partial_current_content_visible` | Hot set, warmed roots, partial restore, partially fetched provider rows. | `Partial` plus covered and omitted scope. | Complete for requested scope. |
| `mixed_cached_and_live_visible` | Some rows are live and some are cached/stale. | Row-level basis plus aggregate mixed-state cue. | One aggregate "current" label. |
| `policy_hidden_prior_content` | Prior content exists but policy withholds it. | Policy-blocked cue and allowed fallback. | Empty, no results, or unavailable without policy source. |

Stale and cached prior content stays inspectable when policy allows, but
mutating actions must revalidate, route through review, or remain
disabled with a reason.

## Surface Reuse Rules

### Shell

The shell keeps chrome, palette, focus routing, breadcrumbs, status
items, and local editing paths interactive while content warms. It may
draw a shell skeleton during startup, but any restored placeholder must
name whether it is first-run, warming, partial, stale, blocked, or
missing a dependency. A whole-shell spinner is forbidden.

### Start Center

Start Center empty and first-run states use the same `empty_first_run`
state class and advertise work-resume actions before account, service,
marketplace, or release-marketing content. Missing recent targets render
as stale/cached or missing-target rows, not as fresh recent work.

### Search

Search distinguishes exact no-results from not-yet-loaded results:
`empty_no_results` requires the searched scope to be complete for the
query. `indexing` and `partial_data` rows reuse search-readiness states
such as partial index, hot set ready, stale index, and index unavailable.
No search surface may render zero matches when the engine has not
searched the requested scope.

### Docs / Help

Docs/help surfaces reuse source/version/cache truth from the docs
browser packet. Mirrored, cached, stale, or provider-unavailable docs
keep their pack/source label visible. Browser handoff or live provider
retry actions must not imply the cached result is already current.

### Install Review

Install, update, package, and extension review surfaces use probing,
blocked, degraded, missing-extension/provider, partial-data, and
stale-cached states for registry metadata, compatibility evidence,
permission resolution, activation budgets, rollback posture, and mirror
state. A package card cannot stay "compatible" or "ready to install" if
the only evidence is stale, partial, unverified, or policy narrowed.

### Provider-Backed Surfaces

Provider-backed rows disclose whether they are local drafts, inspect-only
provider snapshots, browser handoff, deferred publish, or immediate
provider mutation. Provider unavailable states may show cached or local
draft content, but must label it as cached, stale, local-only, or
inspect-only before any provider action.

## Denials

A surface must refuse or downgrade rendering when:

- it cannot name a state class;
- it uses `Ready`, `Healthy`, `Passing`, or green success treatment while
  probing, warming, partial, blocked, degraded, missing, stale, cached,
  or provider-limited truth remains;
- a placeholder row looks like real content without a label;
- a skeleton's final content would reflow into a different row anatomy;
- no-results is shown before the requested scope has been searched;
- stale cached content lacks a last-known-good basis or refresh/requery
  action;
- partial data lacks covered and omitted scope labels;
- blocker or policy source is available but hidden behind "Unavailable";
- state is only exposed by hover, tooltip, or support-only export; or
- export flattens captured/cached/stale content into current truth.

## Fixture Acceptance

A fixture under
[`/fixtures/ux/placeholder_cases/`](../../fixtures/ux/placeholder_cases/)
is conforming only when a reviewer can answer:

1. Which canonical state class is rendered?
2. What content basis is visible: empty, skeleton, progressive, cached,
   stale, or retained prior content?
3. What still works, and what is not ready, reduced, blocked, or stale?
4. What label prevents the placeholder from looking ready?
5. Which command-backed next action is safe?
6. Does support export preserve the same state class, basis, labels, and
   stale/cache truth?
