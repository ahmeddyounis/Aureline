# Naming and state label contract

This contract freezes the vocabulary Aureline uses for command names,
settings, panels, views, state chips, reason chips, client-scope labels,
exports, CLI summaries, documentation, accessibility strings, telemetry
dictionaries, and support packets.

It exists so labels stay operational instead of decorative. A reviewer
must be able to reject a name mechanically when it hides the action,
reuses a state word with a different meaning, softens a policy or trust
reason, or makes a browser/headless surface sound like the full desktop
client.

The companion artifacts are:

- [`/schemas/copy/label_term.schema.json`](../../schemas/copy/label_term.schema.json)
  - boundary schema for the controlled glossary and worked review cases.
- [`/artifacts/copy/controlled_glossary.yaml`](../../artifacts/copy/controlled_glossary.yaml)
  - machine-readable controlled terms, alias posture, governance rules,
  and mechanical denial rules.
- [`/fixtures/copy/state_label_cases/`](../../fixtures/copy/state_label_cases/)
  - worked cases covering command rename review, state-chip comparison
  across surfaces, client-scope mapping, and rejected decorative naming.

This contract composes with:

- [`/docs/copy/count_scope_freshness_grammar.md`](./count_scope_freshness_grammar.md)
  for count, scope, freshness, omission, and chronology microcopy.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  for command descriptors, aliases, CLI projection, and surface parity.
- [`/docs/ux/menu_command_bar_contract.md`](../ux/menu_command_bar_contract.md)
  for menu, toolbar, context-menu, and command-bar projection.
- [`/docs/companion/companion_surface_contract.md`](../companion/companion_surface_contract.md)
  for browser companion scope and handoff limits.

If a source contract owns a schema field, lifecycle state, or command
descriptor value directly, that source contract wins. This document owns
the human-facing label contract and must be updated in the same change
when the source value changes.

## Core rule

The same visible word keeps the same meaning everywhere it appears.
`Stale` in the UI, a CLI summary, an export header, docs, telemetry, and
a support packet means the same reserved state: previous data is shown
after its freshness floor or causal continuity was lost. If a team wants
to use that word for another meaning, it must propose a controlled alias
or choose a different label before the change ships.

The default label style is sentence case. Proper nouns, platform labels,
command IDs, schema keys, keyboard tokens, and protocol names keep their
canonical casing.

## Label classes

| Class | Required shape | Mechanical rejection examples |
|---|---|---|
| Command name | Verb-first, outcome-facing, and tied to one command descriptor. Name the object or scope when a command mutates, publishes, deletes, grants, or exports. | `Make it better`, `Magic repair`, `Continue` for a destructive action, a CLI verb that means something different from the palette label. |
| Setting name | Noun phrase naming the controlled value, source, or policy. Avoid imperative phrasing for the setting itself. | `Turn on awesome sync`, `Better security`, `Advanced stuff`. |
| Panel or view title | Stable noun phrase naming the object, workflow, or inspector. Avoid campaign language and vague workspaces. | `Command center`, `Power zone`, `Everything dashboard`. |
| State label | Controlled vocabulary only. A state word has one reserved meaning across product, docs, CLI, exports, support, accessibility, and telemetry. | `Ready` for cached data, `Degraded` for a hard policy block, `Unavailable` when read-only inspection still works. |
| Reason chip | Short cause label that names the owner or boundary: policy, trust, permission, client scope, freshness, missing dependency, unsupported action. | `Oops`, `Try later`, `Blocked` with no owner, `Limited` with no reason. |
| Client-scope label | Closed labels for where a workflow can run: `Desktop`, `Browser companion`, `Desktop + browser companion`, `Headless only`, `Local only`, or a reviewed extension of that set. | `Web IDE` for the companion, `Cloud mode` for managed-only scope, `Desktop only` when the canonical label is `Desktop`. |
| Button label | Verb plus specific object or outcome. Destructive buttons name the destructive result; safe buttons name the safer path. | `Yes`, `No`, `OK`, `Continue`, `Apply` without scope on a consequential change. |

## Command names

Command names describe the user's outcome, not an implementation detail
or internal subsystem. The command descriptor owns the stable command ID,
CLI verb, aliases, lifecycle state, and per-surface projection. This
contract governs the visible phrase reviewers approve.

Rules:

- Start with a verb where practical: `Open folder`, `Rebuild workspace
  index`, `Export support bundle`, `Open in desktop`.
- Name the object or scope for consequential work: `Delete 3 saved
  worksets`, not `Delete`.
- Keep CLI help, command palette rows, menus, docs, automation recipes,
  and support packets mapped to the same descriptor and alias record.
- Retain old labels only as typed aliases. A legacy search alias may
  help users find a renamed command, but it may not become a second
  product label with a different meaning.
- Use `--dry-run`, `preview`, or `review` language only when the command
  actually provides that posture.

Rejected command names include decorative verbs, hidden blast radius,
unclear objects, and surface-local aliases that cannot trace back to one
command descriptor.

## Setting names

Setting labels are noun phrases because the control already supplies the
interaction. The label names the value or policy being configured:

- `Workspace trust`
- `Proxy source`
- `Telemetry retention`
- `AI provider policy`
- `Command palette history`

Toggle states, help text, or button labels may use verbs, but the setting
name itself must not become a marketing claim or setup instruction.

## Panel and view names

Panel and view titles are stable landmarks. They name the object,
workflow, or inspector the user is in:

- `Project Doctor`
- `Execution context`
- `Support bundle preview`
- `Policy history`
- `Command palette`
- `Browser companion`

A panel title may use a product term that is already controlled. It may
not create a local nickname for a governed concept, because that breaks
docs, screenshots, onboarding, support packets, and accessibility
landmarks.

## State labels

The product-wide baseline state labels are:

| Label | Reserved meaning |
|---|---|
| `Ready` | The declared object is usable for the declared scope with current authority. |
| `Warming` | Background preparation is still in progress and a narrower subset may already be usable. |
| `Partial` | Some requested data, scope, or capability is missing, blocked, unloaded, or still computing. |
| `Stale` | Previous data is shown after its freshness floor or causal continuity was lost. |
| `Rebuilding` | Disposable derived state is being recreated; user-authored state remains protected. |
| `Restricted` | Trust, permission, or policy narrows capability while some inspection or local work remains possible. |
| `Policy blocked` | A policy authority denies the action or state transition. |
| `Reconnecting` | A previously live connection is trying to restore continuity. |
| `Degraded` | Partial capability remains; the degraded subset and recovery path must be visible. |
| `Read-only degraded` | Inspection remains available but mutation or execution is unavailable because capability narrowed. |
| `Unavailable` | The surface cannot provide the requested capability in the current context. |
| `Rollback available` | A reviewed recovery path exists for returning to a prior state. |

These labels may be localized, but their meaning may not be exchanged.
If a surface needs a narrower reason, it uses a reason chip next to the
state label rather than inventing a synonym.

## Reason chips

Reason chips explain why a state or action is narrowed. They are short
but not vague. Approved reason families include:

- `Policy blocked`
- `Trust required`
- `Permission required`
- `Desktop required`
- `Client scope excludes surface`
- `Unsupported action`
- `Stale snapshot`
- `Partial data`
- `Read-only`
- `Local only`
- `Not configured`
- `Unavailable in this profile`

Reason chips do not replace state labels. `Read-only degraded` can carry
`Desktop required`; `Restricted` can carry `Trust required`; `Stale` can
carry `Stale snapshot`.

## Client scope labels

Client scope is separate from lifecycle, support class, commercial
entitlement, and trust. A workflow can be stable and still be `Desktop`;
another can be experimental and still be `Headless only`.

| Label | Meaning | Forbidden implication |
|---|---|---|
| `Desktop` | The installed desktop client is the qualified work surface for the workflow. | Does not mean paid, stable, or online. |
| `Browser companion` | Scoped review, triage, docs, approval, or handoff surface. It is not the full IDE. | Does not imply desktop parity or local runtime control. |
| `Desktop + browser companion` | The workflow has a qualified desktop path and a scoped companion path with explicit handoff limits. | Does not let the companion silently widen authority. |
| `Headless only` | CLI, automation, or service execution is the qualified path; no graphical client workflow is promised. | Does not imply no review, schema, or support output. |
| `Local only` | No vendor-hosted copy or managed service path participates in the declared value. | Must not be used when managed recall, sync, or hosted evidence participated. |

Browser and companion labels must appear anywhere a user could mistake a
handoff, mobile, or browser surface for full desktop capability. Exports
and support packets preserve the same client scope users saw.

## Buttons

Button labels use sentence case and name the outcome:

- Safe or reversible: `Open policy details`, `Continue read-only`,
  `Export before delete`, `Open in desktop`.
- Destructive: `Delete worksets`, `Revoke token`, `Discard changes`.

Consequential dialogs and sheets never rely on `Yes`, `No`, `OK`, or
`Continue` as the only decision labels. Destructive labels must match the
destructive styling and focus rules in the design system; safe labels
must not be styled as destructive merely because they sit near risk.

## Glossary ownership

The controlled glossary is owned jointly by product design, docs,
localization, supportability, and the relevant schema or command owner.

Change rules:

1. Adding a canonical term requires a definition, reserved meaning,
   allowed surfaces, forbidden uses, owner role, and at least one fixture
   or existing fixture update.
2. Adding an alias requires the canonical term, alias status, allowed
   surfaces, migration plan, and explicit review rationale.
3. Repurposing a canonical term is breaking and requires a replacement
   term plus migration notes. Do not silently change the meaning of an
   existing label.
4. Localized strings preserve the reserved meaning. Translators may
   reorder grammar, but they may not soften policy, trust, recovery,
   client-scope, or degraded-state meanings.
5. Product UI, CLI, documentation, exports, accessibility strings,
   telemetry dictionaries, and support packets use the same glossary row
   for the same state.

## Alias handling by domain

Aliases exist to support migration, localization, or platform wording.
They do not create second meanings.

| Domain | Alias rule |
|---|---|
| Trust | Preserve the trust boundary and the remaining safe path. An alias for `Trust required` or `Restricted` must not sound like a generic preference. |
| Policy | Preserve the policy owner or authority. An alias for `Policy blocked` must not become `Unavailable`, `Try later`, or another non-owner phrase. |
| Recovery | Preserve the recovery class and scope. An alias for `Rollback available` must not imply exact undo when recovery is partial or compensating. |
| Evidence | Preserve freshness and proof posture. An alias for `Evidence stale`, `Retest pending`, or `Stale snapshot` must keep the timestamp or refresh route available. |
| Support class | Preserve claim strength. `Certified`, `Supported`, `Limited`, `Community`, and `Experimental` cannot be replaced with warmer marketing labels. |
| Client scope | Preserve the qualified client. `Browser companion` cannot alias to `Web IDE`, and `Local only` cannot alias to `Offline` when data-hosting truth matters. |
| Degraded states | Preserve remaining capability. An alias for `Degraded` or `Read-only degraded` must state what still works and what does not. |

## Mechanical review

Reviewers can reject a label without taste debate when any of these are
true:

- A command is not verb-first or hides the affected object.
- A setting label is an imperative, slogan, or vague adjective.
- A panel or view title is a nickname that cannot map to a controlled
  term or product object.
- A state word is reused with a different meaning.
- A reason chip omits the responsible boundary.
- A client-scope label implies desktop parity, managed service
  availability, or support maturity that the evidence does not prove.
- A destructive button uses generic confirmation copy.
- A term appears in one surface but lacks CLI, docs, export, accessibility,
  telemetry, or support-packet parity where that surface emits those
  artifacts.

The fixture set exercises these rejection paths and is the minimum seed
corpus for future automated checks.
