# Language three-layer model and semantic-depth truth contract

This document freezes Aureline’s **three-layer language intelligence model**
and the **truth contract** that prevents “supports language X” from becoming
an uninspectable mix of syntax, protocol compatibility, and IDE-owned
semantics.

The goal is not to define a “best” implementation, but to force every user-
visible surface to answer two questions consistently:

1. **Which layer answered this?**
2. **If a deeper layer exists, why wasn’t it used here?**

Machine-readable companions:

- [`/artifacts/language/layer_matrix.yaml`](../../artifacts/language/layer_matrix.yaml)
  — the frozen layer vocabulary, feature-family mapping, and downgrade rules.
- [`/fixtures/language/layer_cases/`](../../fixtures/language/layer_cases/)
  — worked cases that show required layer labeling and downgrade disclosure.

This contract composes with and does not replace:

- [`/docs/architecture/language_protocol_router_adr.md`](../architecture/language_protocol_router_adr.md)
  — provider capability negotiation and routing across syntax, LSP/DAP,
  formatter/linter/test/build adapters, graph, framework packs, and native
  analyzers.
- [`/docs/language/provider_graph_and_arbitration_contract.md`](./provider_graph_and_arbitration_contract.md)
  — result provenance, alternate-provider disclosure, freshness/scope honesty,
  and downgrade cues at the provider level.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — cross-surface degraded-state tokens and “worst-supporting-truth-wins”.
- [`/docs/search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  — readiness/freshness vocabulary for search, navigation, and graph overlays.
- [`/docs/navigation/semantic_navigation_and_rename_contract.md`](../navigation/semantic_navigation_and_rename_contract.md)
  — durable semantic-result identity and rename-preview truth contract.
- [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md)
  and
  [`/docs/execution/test_truth_contract.md`](../execution/test_truth_contract.md)
  — debug/test truth rows that must also disclose the answering layer.

If this document disagrees with the PRD, technical architecture, technical
design, or any linked contract above, those documents win and this document,
the matrix, and the fixtures update in the same change.

## Why freeze this now

Language intelligence is a high-risk source of accidental product truth:

- syntax-only structure can render like semantic certainty;
- compatibility (LSP/DAP) answers can vary wildly by server yet look “first-
  party”;
- Aureline-owned semantic depth (graph + packs + analyzers) can be partial,
  warming, stale, or policy-blocked, and must not silently disappear;
- AI surfaces can blend syntax, compatibility, and semantic depth into one
  confident paragraph unless the product forces attribution.

The three-layer model exists so “language support” becomes inspectable and
degradable instead of a single opaque badge.

## Definitions

### The three layers

The language intelligence layer is a *truth depth classifier*, not a
deployment topology.

| Layer | Layer id | Canonical job | Typical sources | Never imply |
|---|---|---|---|---|
| Layer 1 | `layer_1_syntax_structure` | Syntax and structure | Tree-sitter parse, tokenization, structural selection/folds/breadcrumbs, grammar-local error recovery | workspace-wide semantics, type/project awareness, “safe refactor” |
| Layer 2 | `layer_2_compatibility_breadth` | Compatibility and breadth | LSP (hover, completion, diagnostics, rename, references, formatting, inlay hints, semantic tokens), DAP (debug), toolchain adapters (formatter/linter/test/build) | Aureline-owned semantic certainty, uniform quality across languages, framework/build-aware correctness |
| Layer 3 | `layer_3_aureline_semantic_depth` | Aureline-owned semantic depth | semantic workspace graph, framework packs, native analyzers, build-target awareness, ownership/docs linking, impact/refactor assistance beyond raw LSP | “complete” when scope is partial/warming/stale, or when evidence/graph freshness is below the surface’s truth floor |

### Answering layer

Every protected surface that consumes language intelligence **MUST** classify
its answer as exactly one `answer_layer_id`:

- If the answer is produced solely from syntax/structure, it is Layer 1.
- If the answer’s authoritative claims come from compatibility providers
  (LSP/DAP/toolchain adapters) without Aureline-owned semantic proof, it is
  Layer 2.
- If the answer’s authoritative claims are grounded in Aureline-owned semantic
  depth (graph/packs/analyzers), it is Layer 3 even if Layer 2 contributes
  overlays.

Surfaces may display contributing providers, but they must still pick one
answering layer so user expectations don’t collapse into “some kind of smart”.

### Deeper layer unavailable

When a surface answers below the deepest possible layer for that feature
family, it **MUST** disclose why the deeper layer was not used. Reasons must
be expressed using the shared degraded-state and provider-health vocabulary
(for example: warming, partial scope, stale, offline, policy blocked,
unsupported, quarantined).

## Feature-family mapping (what each layer can honestly claim)

This section is a human-readable view of the matrix in
[`/artifacts/language/layer_matrix.yaml`](../../artifacts/language/layer_matrix.yaml).

### Hover

- Layer 1: structural hover (token kind, syntax node class, doc-comment
  extraction where grammar-local).
- Layer 2: LSP hover payloads, signature help overlays, protocol-derived docs.
- Layer 3: graph-backed symbol identity, framework-aware facts, build-aware
  type resolution, doc/ownership linking with freshness and scope labels.

If hover falls back to Layer 1 or Layer 2, the UI **MUST** label the answer as
syntax-only or compatibility-only and provide a “why not semantic depth” path.

### Completion

- Layer 1: keyword/grammar-based completions and snippet-only completion.
- Layer 2: LSP completion lists, signature help, basic ranking from provider.
- Layer 3: graph-aware ranking, framework-aware suggestions, project-target
  awareness, safe-import insertion rules, and history-aware suppression that
  does not falsify provenance.

Completion **MUST** preserve source labeling (syntax vs compatibility vs
semantic depth) so “deep understanding” does not render like “fast guessing”.

### Diagnostics and code actions

- Layer 1: parse/grammar recovery diagnostics and structure errors.
- Layer 2: LSP diagnostics, linter diagnostics, build/test adapter imports, and
  protocol quick-fix/code-action proposals.
- Layer 3: graph-normalized finding taxonomy, framework-aware diagnostics,
  cross-file causal linking, and impact-aware safe fix proposals.

Diagnostics surfaces **MUST** avoid collapsing Layer 2 and Layer 3 findings
into one unlabeled list; each cluster must remain layer-attributable and obey
the downgrade rules in the diagnostics contracts.

### Rename / references / navigation

- Layer 1: text/syntax-local navigation and file-local rename *only* with
  explicit scope warnings.
- Layer 2: LSP definition/references/rename where the server claims authority.
- Layer 3: durable semantic identity backed by the workspace graph, with scope
  completeness and preview/rollback semantics for any multi-file change.

When rename or refactor falls back to Layer 1 or Layer 2, the product **MUST**
label the downgrade (text-only or compatibility-only) and require preview and
scope disclosure before applying changes.

### Formatting / inlay hints / semantic tokens

- Layer 1: minimal indentation or structural formatting that never claims style
  parity; basic token classes from syntax queries.
- Layer 2: LSP formatting/inlay hints/semantic tokens and formatter adapters.
- Layer 3: framework- and project-aware formatting safety (generated artifacts,
  embedded languages, notebook cells), semantic tokens tied to durable symbol
  identity, and mutation gating when mapping quality is insufficient.

### Debug and tests

- Layer 2: DAP debug sessions; test discovery/execution via adapters.
- Layer 3: build-target and artifact selection grounded in the workspace graph;
  debug/test items linked to code ownership, symbols, and impact scope with
  freshness and provenance.

Debug/test UX **MUST** disclose when it is running as pure compatibility
(adapter-only) versus graph-backed (artifact/target identity is proven).

### Graph overlays / impact analysis / refactor assistance

These are Layer 3 by definition. If a surface offers them at all, it must
either:

- provide them with Layer 3 provenance; or
- explicitly label the result as heuristic/advisory and route to the
  appropriate degraded-state explanation (never presenting it as semantic
  certainty).

## Cross-surface rules (search, navigation, docs, review, AI)

1. **Search and navigation** MUST expose the answering layer for every result
   row that claims semantic meaning (symbol, reference, ownership, impact).
   If a result is syntax-only or compatibility-only, it must be labeled and
   its scope completeness must be explicit.
2. **Refactor surfaces** MUST treat Layer 1 and Layer 2 answers as downgrade
   modes for any semantic refactor class, forcing preview, scope labeling, and
   rollback posture. Layer 3 is the only layer that may claim “semantic
   refactor assistance”.
3. **Docs and onboarding** MUST not say “supports language X” without naming
   the layer(s) supported. Any published claim about “semantic depth” must be
   traceable to Layer 3.
4. **Review surfaces** MUST keep findings and suggestions layer-attributed so
   reviewers can distinguish syntax-only evidence from compatibility output and
   from graph-backed semantic certainty.
5. **AI surfaces** MUST preserve layer provenance for every cited fact or
   suggestion. AI output must not “upgrade” a Layer 1/2 fact into a Layer 3
   claim; it may only summarize or propose, with provenance and degraded-state
   tokens intact.

## Conformance checklist

A change is conforming if:

- every language-adjacent surface can explain its answers via exactly one
  `answer_layer_id`;
- downgrade paths from Layer 3 → 2 → 1 are explicit and use shared
  degraded-state vocabulary;
- product and docs claims about “semantic depth” map to Layer 3 only;
- language teams can extend capabilities by updating the matrix + fixtures,
  instead of creating a surface-local “truth model” in prose.

