# Language Protocol Router, Capability Arbitration, And Provider Quarantine ADR Seed

- **Decision id:** pending formal register row
- **Status:** Accepted
- **Decision date:** 2026-04-26
- **Owner:** `@ahmeddyounis`
- **Forum:** architecture_council
- **Related requirement ids:** `none`

## Context

Aureline's language stack is deliberately hybrid. Tree-sitter supplies
syntax and structure, LSP and DAP provide broad ecosystem compatibility,
formatter / linter / test / build adapters connect to the toolchain, and
Aureline-owned graph, framework, and native analyzer lanes provide the
depth that generic protocol adapters cannot guarantee.

That layering is valuable only if it converges through one router. If
LSP, DAP, formatters, test adapters, build adapters, framework packs,
and native analyzers each decide precedence, fallback, health, placement,
and coordinate translation privately, the product will show conflicting
truth: a diagnostic might look current while its build adapter is stale,
a formatter might silently replace a framework-safe formatter, a debug
launch might ignore the build graph that selected the artifact, or a
quarantined provider might disappear without explaining which features
were narrowed.

This seed freezes the router contract and its companion boundary
schemas:

- [`/schemas/language/provider_capability.schema.json`](../../schemas/language/provider_capability.schema.json)
  defines the provider capability descriptor every language, debug,
  formatter, test, build, framework, native analyzer, project-graph,
  generated-source bridge, and assist provider advertises.
- [`/schemas/language/provider_resolution.schema.json`](../../schemas/language/provider_resolution.schema.json)
  defines the resolution record emitted when a surface asks the router
  to pick, merge, overlay, or reject providers for one capability.
- [`/fixtures/language/router_cases/`](../../fixtures/language/router_cases/)
  contains worked cases for completion fallback, diagnostics
  coexistence, formatting precedence, test discovery fallback, debug
  quarantine, and framework hover overlay conflicts.

This contract composes with:

- [`/docs/language/three_layer_model_contract.md`](../language/three_layer_model_contract.md)
  for the user-facing “which layer answered?” depth labeling that sits above
  provider-level arbitration.
- [`/docs/language/provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md)
  for language-result provenance, scope honesty, alternate-provider
  disclosure, and framework certainty.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  for diagnostic convergence and mutation-safety disclosure.
- [`/docs/language/completion_and_inline_hint_contract.md`](../language/completion_and_inline_hint_contract.md)
  for completion, signature-help, snippet, and inline-hint source
  disclosure.
- [`/docs/execution/run_and_attempt_contract.md`](../execution/run_and_attempt_contract.md)
  for task-like lifecycle, queue admission, rerun, artifact, and
  cancellation truth.
- [`/docs/execution/test_truth_contract.md`](../execution/test_truth_contract.md)
  for test discovery, item state, flake, and quarantine semantics.
- [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md)
  for debug session posture, artifact mapping quality, and restart /
  reattach authority.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  for crash-loop supervision, quarantine, forensic packets, and visible
  degraded states.
- [`/docs/architecture/parser_substrate_adr.md`](parser_substrate_adr.md)
  for canonical editor coordinates and protocol coordinate projections.

If this ADR disagrees with the PRD, technical architecture, technical
design, UI/UX spec, or the linked contracts above, those documents win
and this ADR plus its companion schemas update in the same change.

## Decision

Aureline will route every language-adjacent provider capability through
one typed language protocol router. LSP, DAP, formatters, linters, test
adapters, build adapters, framework packs, native analyzers, project
graph providers, generated-source bridges, syntax providers, and assist
providers advertise capabilities through the same
`provider_capability_record` shape. Every user-visible assist or tooling
result that depends on those capabilities emits a
`provider_resolution_record` explaining which provider answered, why it
won or coexisted, which providers were unavailable, unsupported,
fallback-only, blocked, or quarantined, and which degraded state or
coordinate translation requirement affected the answer.

## Router Scope

The router owns arbitration for these capability classes:

| Capability class | Typical producers | Required result disclosure |
|---|---|---|
| `diagnostics` | LSP, compiler/build adapter, linter, framework pack, native analyzer | origin provider set, merge/coexistence rule, stale or conflicting contributors, unsupported classes |
| `completion` | LSP, project graph, local index, snippets, framework pack, native analyzer, assist provider | selected provider, ranking/provenance label, fallback or advisory status |
| `hover` | LSP, project graph, framework pack, native analyzer, generated bridge, assist provider | primary factual source, overlay contributors, conflict class when prose or framework fact disagrees |
| `formatting` | LSP, formatter adapter, framework formatter, native formatter, fallback text formatter | precedence reason, fallback edit safety, unsupported or policy-blocked formatter classes |
| `test_discovery` | native test adapter, build adapter, framework pack, structured importer, heuristic parser | discovery scope, freshness, provider origin, fallback confidence |
| `debug_launch` / `debug_attach` / `debug_session_control` | DAP adapter, native debug bridge, build adapter, artifact manager, framework pack | adapter origin, artifact/build binding, launch support, quarantine or inspect-only status |
| `build_target_discovery` / `build_diagnostics` | native build adapter, BSP, BEP/BES stream, structured importer, heuristic parser | target graph source, confidence tier, omitted roots, stale target graph posture |
| `framework_navigation` / `framework_run_scaffold` | framework pack, project graph, build adapter, runtime evidence | framework certainty, proving artifact refs, fallback to inspect-only when support is partial |
| `coordinate_translation` | parser substrate, generated bridge, source-map bridge, notebook adapter | coordinate profiles, mapping quality, mutation gate when mapping is missing or approximate |

The existing language-provider arbitration contract remains the
surface-level record for navigation, hover, completion, and code-action
results. This router sits one layer above it and also covers execution
and toolchain providers so language bundles, framework packs, and
extension hosts reuse the same capability model.

## Provider Capability Record

A provider capability descriptor answers:

1. What provider family and protocol family is being advertised?
2. Where does the provider run: in-process, local sidecar, remote agent,
   managed service, or imported snapshot?
3. Which capabilities does it support as authoritative, advisory,
   fallback-only, inspect-only, or unsupported?
4. What precedence band, coexistence mode, and fallback path applies for
   each capability?
5. What health state, health score, freshness, restart budget, and
   quarantine posture applies now?
6. What scope can the provider honestly cover?
7. What coordinate profile does the provider speak, and when must
   translation happen before request, result rendering, or mutation?
8. What source/provenance record made this provider available?

Rules:

1. A provider may advertise multiple capabilities, but health and
   placement are not global permission to use every capability. The
   router evaluates each capability row independently.
2. `crash_loop_quarantined` is a hard exclusion for new primary
   selection. A quarantined provider remains inspectable and may appear
   in explanations, but it cannot silently win.
3. A provider with `support_class = unsupported` must name an
   unsupported reason. Unsupported is not the same as temporarily
   unavailable.
4. A provider whose coordinate translation requirement is
   `blocked_until_mapping_available` cannot perform mutation, broad
   formatting, breakpoint binding, rename, or source-jump actions until
   the mapping record exists.
5. A remote provider's answer is not weaker merely because it is remote,
   but its result must carry placement and execution-context identity so
   local, remote, and managed routes remain explainable.
6. Assist providers are advisory or fallback-only by construction. They
   may rank, summarize, or suggest, but they cannot satisfy an
   authoritative-required request without deterministic provider
   evidence.

## Resolution Record

Every protected surface request emits a provider resolution record before
rendering the answer as product truth. The record includes:

- request context: surface, capability class, subject ref, requested
  scope, requested authority floor, placement preference, and coordinate
  requirement;
- candidate rows copied from provider capability descriptors at
  resolution time;
- selected provider rows, contributing provider rows, excluded provider
  rows, unavailable provider rows, and quarantined provider rows;
- selection reason, resolution mode, precedence band, health score,
  degraded state, conflict class, and fallback class;
- coordinate translation result and mapping quality when any provider
  speaks external protocol coordinates;
- a surface-report block with the origin label, unsupported capability
  classes, degraded-state label, and export-safe explanation.

Resolution modes:

| `resolution_mode` | Meaning |
|---|---|
| `single_winner` | One provider is the primary answer. Alternates remain inspectable. |
| `ordered_fallback` | Primary provider was unavailable or below the requested floor, so a lower-priority provider answered with a degraded label. |
| `merged_coexistence` | Multiple providers contribute to one surface without hiding source identity, such as compiler plus LSP diagnostics. |
| `overlay` | One provider overlays another, such as framework hover or generated-source mapping over LSP facts. |
| `inspect_only_no_winner` | No provider may answer authoritatively, but the router can explain why. |
| `unsupported` | No advertised provider supports the requested capability for this scope or host. |

Rules:

1. Candidate order is not product truth. The resolution record must name
   the selection reason instead of relying on array order.
2. A higher-precedence provider may lose when its health score,
   freshness, scope, policy, coordinate mapping, or placement is below
   the request floor. The losing reason stays visible.
3. Diagnostics commonly use `merged_coexistence`; formatting and debug
   launch commonly use `single_winner`; framework hover commonly uses
   `overlay`; test discovery commonly uses `ordered_fallback` when a
   native adapter is missing or stale.
4. The router may preserve multiple providers for a result, but it may
   not collapse conflicting providers into one unlabeled source. A
   non-`none` conflict class requires at least one visible contributing
   or excluded provider row.
5. A cached or heuristic fallback never upgrades to the original
   provider's authority. It keeps its own fallback and confidence label.
6. Unsupported classes remain first-class: "no formatter here" and
   "formatter crashed" are distinct states and drive different recovery
   paths.

## Precedence And Coexistence

Default precedence is a deterministic seed, not a substitute for health
and policy checks.

| Precedence band | Intended use |
|---|---|
| `first_party_native` | First-party native analyzers, native build/test adapters, native formatters, and native debug bridges for claimed launch stacks. |
| `framework_overlay` | Framework pack facts that refine language or build truth while citing proving artifacts. |
| `project_graph_authority` | Shared semantic workspace graph, target graph, and indexed symbol facts. |
| `protocol_compatibility` | LSP, DAP, BSP, and other standard protocol adapters. |
| `structured_tool_adapter` | Structured output importers, formatter CLIs, linter CLIs, test report importers, and build event streams. |
| `imported_snapshot` | Imported CI, support bundle, or cached provider state. |
| `heuristic_fallback` | Grep, problem matchers, line-oriented parsers, local text fallback, or inferred test discovery. |
| `assist_only` | AI or assistant lanes that can explain or suggest but not own deterministic truth. |

Coexistence is declared per capability:

- diagnostics merge only when each contributing finding keeps source,
  evidence plane, freshness, and semantic-layer state;
- hover overlays are allowed when primary factual source and narrative
  or framework overlay remain separate;
- formatting is exclusive by default because two formatters applying
  overlapping edits is unsafe unless a future formatter pipeline proves
  non-overlap;
- test discovery may merge native and provider-imported trees only when
  scope, freshness, and parity labels survive;
- debug launch is exclusive for the adapter that owns the session, but
  build, artifact, and framework providers may contribute prerequisites
  and inspect-only explanation rows.

## Health Scoring And Quarantine

Health has two axes:

- `health_state`, the coarse user-visible state; and
- `health_score`, a bounded 0-100 score used only inside the router and
  exported for support.

The router calculates health from restart strikes, recent successful
answers, latency/budget violations, stale epochs, policy blocks,
placement reachability, and coordinate-mapping availability. The exact
algorithm may evolve, but the record fields and exclusion rules are
frozen:

1. `ready` providers may still lose if another provider has higher
   precedence or better scope for the requested capability.
2. `warming`, `degraded`, and `cached_only` providers may answer only
   when the requested authority floor allows downgrade or when the
   surface is inspect-only.
3. `policy_blocked`, `capability_missing`, `crash_loop_quarantined`,
   and `unavailable` providers cannot be selected as primary.
4. Crash-loop quarantine is scoped to the provider and capability family
   declared by the supervisor. Quarantining one debug adapter must not
   collapse completion, diagnostics, or formatting into an ambiguous
   language failure.
5. A quarantine explanation must name the affected provider, the
   excluded capability classes, the fallback or unsupported result, and
   the recovery owner.

## Coordinate Translation

Aureline editor coordinates remain canonical. External protocol
coordinates are projections:

- LSP uses UTF-16 positions.
- DAP uses source line/column and debug artifact mappings.
- Formatters may use whole-document, byte, line, or text-span ranges.
- Test and build output often carries path/line/column locations.
- Framework and generated-source bridges may use source-map or
  generated/original spans.
- Native analyzers may use compiler-specific span encodings.

Rules:

1. Every provider capability row declares its input and output
   coordinate profiles.
2. The router blocks mutation when coordinate translation is required
   but no exact or remapped mapping record exists.
3. Approximate mappings may support inspect-only navigation or
   explanation, but they cannot support broad refactor, formatter,
   breakpoint, or fix-all authority.
4. Generated-source, notebook, source-map, and debug-artifact mappings
   remain additive context; they do not replace canonical source
   identity.

## Surface Reporting Rules

Every compact UI, CLI, support export, and AI evidence packet that uses
router output must preserve at least:

- provider origin label;
- selected resolution mode;
- degraded state, if any;
- unsupported capability classes, if any;
- unavailable or quarantined providers that affected the answer;
- coordinate translation status when it affected source location,
  formatting, breakpoint binding, or mutation;
- export-safe explanation that reconstructs why this answer appeared.

Surface-specific minimums:

- Diagnostics report each contributing provider and never hide compiler,
  LSP, linter, framework, native analyzer, runtime/test, or imported
  provenance inside one unlabeled row.
- Completion reports source kind and advisory/fallback status; AI
  ranking or AI generation remains advisory.
- Hover reports the primary factual source and any framework or assist
  overlay; conflicts stay visible.
- Formatting reports the formatter that would apply edits, why it won,
  and why alternates are blocked, unsupported, or fallback-only.
- Test discovery reports discovery source, scope, freshness, and whether
  fallback is native, imported, structured, heuristic, or provisional.
- Debug launch, attach, and session actions report the adapter, artifact
  identity, prerequisite providers, mapping quality, and any
  quarantine/inspect-only state.

## Consequences

- Language bundles, framework packs, extension hosts, and tool adapters
  must declare router capability rows before they can be treated as
  product truth.
- Existing language result-provenance records remain valid; they can
  cite router resolution ids rather than inventing a second provider
  selection story.
- Build/test/debug adapters now share the same arbitration language as
  editor intelligence, which keeps target graphs, diagnostics, tests,
  debug launches, and framework routes from diverging silently.
- Support bundles can reconstruct provider selection without raw
  provider logs or workspace content.
- A future implementation still needs concrete scoring weights,
  supervisor wiring, and per-language bundle manifests, but those
  implementations must populate the schemas frozen here.

## Alternatives Considered

- **Per-surface arbitration.** Rejected because diagnostics, completion,
  formatting, test, build, and debug would invent incompatible fallback
  and quarantine semantics.
- **Trust protocol providers directly.** Rejected because LSP, DAP,
  formatter, and test protocols solve transport or payload shape, not
  product truth, provenance, or fallback honesty.
- **Use only the existing language-provider contract.** Rejected because
  that contract is intentionally editor-result focused and does not
  cover build target discovery, test discovery, debug launch/session
  control, formatter exclusivity, or native analyzer placement.
- **Let framework packs own routing.** Rejected because framework packs
  enrich graph and execution truth; they must not become isolated
  state machines with private debug/test/build routing.

## Source Anchors

- `.t2/docs/Aureline_PRD.md:152` — "LSP, tree-sitter, DAP, and where
  they stop."
- `.t2/docs/Aureline_PRD.md:156` — LSP has broad coverage but uneven
  framework, build, refactoring, and project-model fidelity.
- `.t2/docs/Aureline_PRD.md:164` — the recommended stack includes
  IDE-owned project graph and framework analyzers.
- `.t2/docs/Aureline_PRD.md:873` — LSP is the broad language coverage
  layer.
- `.t2/docs/Aureline_PRD.md:874` — DAP is the debugger integration
  layer.
- `.t2/docs/Aureline_PRD.md:919` — external extension hosts include
  formatters and linters.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2119` —
  language compatibility includes LSP for hover, completion,
  diagnostics, rename, references, formatting, inlay hints, and semantic
  tokens.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2120` — DAP
  covers launch and attach debugging.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2135` — the
  language router selects the right provider stack per file, root, and
  target.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2173` — the
  router should converge project graph truth for diagnostics, build
  targets, tests, route intelligence, and execution-context selection.
- `.t2/docs/Aureline_Technical_Design_Document.md:2106` — the design
  stack includes LSP, DAP, test, and formatter adapters.
- `.t2/docs/Aureline_Technical_Design_Document.md:2113` — the router is
  responsible for provider selection and capability negotiation.
- `.t2/docs/Aureline_Technical_Design_Document.md:2304` — build adapter
  descriptors must expose adapter kind, capability flags, discovery
  source, and workspace scope.
- `.t2/docs/Aureline_Technical_Design_Document.md:2312` — adapter
  priority is a product contract and must remain visible in diagnostics
  and support exports.
- `.t2/docs/Aureline_Technical_Design_Document.md:2601` — debug launch
  profiles resolve adapter ref/class, execution context, and supported
  verbs through runtime capability negotiation.

## Linked Artifacts

- Capability schema:
  [`schemas/language/provider_capability.schema.json`](../../schemas/language/provider_capability.schema.json)
- Resolution schema:
  [`schemas/language/provider_resolution.schema.json`](../../schemas/language/provider_resolution.schema.json)
- Fixture corpus:
  [`fixtures/language/router_cases/`](../../fixtures/language/router_cases/)

## Supersession History

None.
