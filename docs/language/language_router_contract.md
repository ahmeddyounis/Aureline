# Language router decision, supervision, and fallback-honesty contract

This document freezes how Aureline’s language router selects, supervises, and
explains provider stacks so that **multi-root workspaces**, **remote/local
placement**, and **degraded providers** never create silent contradictions.

The router is not “a list of providers in order”. It is a decision engine that
binds a user-visible request to:

1. **Scope and roots** — which workspace/workset/root(s) the request is scoped
   to, and which root the subject belongs to.
2. **Config + package/workspace roots** — which configuration root and package
   (or workspace) root govern semantics for the subject when those concepts
   exist (for example: language project model roots, package-manager workspaces,
   framework workspaces).
3. **Target + toolchain identity** — which execution-context/toolchain identity
   anchors the semantics (especially for build/diagnostics/debug/test and for
   remote workspaces).
4. **Provider stack** — the eligible provider lanes, their roles (primary,
   contributor, overlay, fallback), and the fallback path.
5. **Supervision posture** — fault-domain identity, restart-budget state, and
   crash-loop/quarantine posture for the providers involved.
6. **Fallback honesty** — which guarantee was lost, what remains safe, and why.

This freeze exists so later implementation work cannot “quietly”:

- substitute a provider after a crash loop and keep rendering results as if
  nothing changed;
- widen scope from one root/package to multiple roots/packages without
  disclosure;
- mix local and remote providers without naming which toolchain/config root is
  in effect;
- serve stale or cached results with live-looking presentation.

Machine-readable companions:

- [`/schemas/language/router_decision.schema.json`](../../schemas/language/router_decision.schema.json)
  — `router_decision_record`, the cross-surface packet that carries routing
  context (roots, config/package roots, target/toolchain identity), provider
  stack selection, supervision posture, and an export-safe explanation.
- [`/fixtures/language/router_cases/`](../../fixtures/language/router_cases/)
  — worked YAML cases for router decisions (in addition to the existing
  provider capability/resolution fixtures).

This contract composes with and does not replace:

- [`/docs/architecture/language_protocol_router_adr.md`](../architecture/language_protocol_router_adr.md)
  — provider capability descriptors and per-capability arbitration results
  (`provider_capability_record`, `provider_resolution_record`).
- [`/docs/language/provider_graph_and_arbitration_contract.md`](./provider_graph_and_arbitration_contract.md)
  — provider-family provenance, scope/freshness honesty, and alternate-provider
  disclosure for language-intelligence results.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  — supervision tree, fault domains, restart budgets, quarantine escalation, and
  forensic packets. Router supervision fields reuse that vocabulary.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  — target/toolchain identity vocabulary; router decision packets must be able
  to explain which execution context anchored a decision.
- [`/docs/workspace/scope_truth_packet.md`](../workspace/scope_truth_packet.md)
  — workset/scope identity and multi-root scoping rules.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — cross-surface degraded-state tokens and “worst-supporting-truth-wins”
  composition.

If this document disagrees with the PRD, technical architecture, technical
design, or UI/UX spec, those documents win and this document plus the schema
and fixtures update in the same change.

## Scope

Frozen at this revision:

- one router decision packet shape (`router_decision_record`);
- required explanation fields for provider choice, scope/root context, config
  root, package/workspace root, toolchain identity, routing reason, and fallback
  cause;
- supervision vocabulary reuse: fault-domain ids, restart budgets, strike
  counts, and quarantine references;
- stale-vs-live labeling rules for local, remote, and mixed stacks.

Out of scope:

- implementing the router, provider hosts, semantic services, or restart
  supervisor logic;
- final UI copy (the vocabulary is frozen; product copy lives with the shell
  interaction and truth vocabulary contracts).

## Definitions

### Router decision

A **router decision** is the exportable binding between:

- a requested surface/capability and subject ref,
- the scope and root context,
- the config and package/workspace roots that govern semantics,
- the execution context (target/toolchain identity),
- the selected provider stack and fallback path,
- and the supervision posture affecting eligibility.

The router decision is **not** just “which provider won”; it includes the
context that makes that outcome explainable and debuggable.

### Config root vs package/workspace root

The router distinguishes:

- **config root** — the root that governs the language project model for a
  subject (for example: a TypeScript config root; a framework config root; a
  generated mapping root).
- **package root** — the package or module boundary the subject belongs to
  (for example: a workspace member package).
- **workspace root** — the top-level root the user admitted into the current
  workset/workspace.

These are opaque identifiers in packets. Surfaces may render short labels, but
they must be able to surface the ids and explain why they were chosen.

## Router decision emission rules

1. **No silent routing on protected surfaces.** Any protected surface that
   presents language-derived truth (navigation, refactor preview/apply, docs
   surfaces such as hover, diagnostics, completion, code actions, tests, debug)
   MUST emit a `router_decision_record` before rendering a result as current
   truth.
2. **One packet can explain the outcome.** A consumer MUST be able to explain:
   provider choice, config root, target/toolchain identity, and fallback path
   from the decision record without scraping logs or inferring from provider
   order.
3. **Provider substitution is always explicit.** If a different provider answers
   than the one that would win in nominal health (because of restart budget,
   crash-loop quarantine, policy block, remote disconnect, missing coordinate
   mapping, or scope narrowing), the decision MUST:
   - set a non-`none` degraded/fallback token, and
   - include an export-safe `fallback_summary` naming what guarantee changed.
4. **Stale/cached results never look live.** If any selected provider is not
   `freshness_class = authoritative_live`, the decision MUST project a stale or
   cached label and must not allow a consumer to render “current” posture
   without an explicit downgrade.
5. **Supervision is not folklore.** Crash loops, strike counts, and restart
   budgets are first-class: the decision packet MUST carry the relevant
   `fault_domain_id`, `restart_budget_ref`, and strike counts for selected and
   fallback providers when known.

## Local, remote, and hybrid provider lanes

Routing must remain explainable when:

- providers run locally in-process,
- providers run in local sidecars,
- providers run in a remote workspace agent,
- providers are served by a managed service lane, or
- a decision mixes more than one of the above.

Rules:

1. **Placement is part of the explanation.** The decision packet MUST include a
   placement summary and locality class for every selected provider in the
   stack.
2. **Remote is not “worse”; it is different.** A remote provider is not
   downgraded for being remote, but it MUST carry target/toolchain identity so
   “which toolchain/config root produced this?” is answerable.
3. **Hybrid must name the boundary.** If a decision is hybrid (mixing local and
   remote lanes), the decision packet MUST declare a hybrid lane posture and
   explain which parts of the answer came from which locality.

## Change management

- Adding a new enum value or a new optional field is additive-minor and bumps
  `router_decision_schema_version`.
- Repurposing an existing enum value or changing field meaning is breaking and
  requires a new decision row plus a schema version bump with a migration note.

