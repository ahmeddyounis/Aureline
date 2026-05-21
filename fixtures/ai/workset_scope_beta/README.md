# AI workset / scope beta corpus

Frozen corpus that promotes the AI composer / context-inspector surface's
workset / sparse-slice / policy-limited-view truth from incidental coverage to
beta. It proves the AI surface honours a declared scope boundary the same way
the workspace, search, graph, and refactor surfaces do: in-scope context stays
drawn, context that escapes the scope is **labeled** (blocked, with a typed
omission reason), policy-hidden members are **disclosed** through the scope
truth — never silently drawn in or silently dropped — and the same scope truth
flows into the AI evidence handoff and spend receipt.

## What each case asserts

Each `*_scope.json` case is replayed by
`crates/aureline-ai/tests/workset_scope_beta.rs`. The test deserializes the
case's `artifact` directly into an `aureline_workspace::WorksetArtifactRecord`,
validates it, and projects an `aureline_workspace::WorksetScopeBetaTruth` for
the `Ai` consumer surface. It then builds an
`aureline_ai::ComposerContextAlphaSnapshot` (the AI composer / context
inspector) whose `scope_label` is derived from the workspace `ScopeClass`
chip-label vocabulary, draws the declared in-scope / out-of-scope /
policy-limited context rows, and asserts:

- the AI `scope_label` is derived from the shared `ScopeClass` vocabulary, not a
  private label set (`ai_scope_class` resolves through the manifest map onto a
  real workspace `ScopeClass`);
- every in-scope context row is `included`/`pinned` in a local locality, and
  every out-of-scope row is `blocked` + `outside_current_scope` +
  `scope_excluded`, and every policy-limited row is `blocked` + `policy`
  (labeled, not silently drawn or dropped) — the snapshot validates clean;
- the scope truth narrows or blocks `ai_apply` for the active scope class
  (`narrowed_to_scope` for named workset / sparse slice, `blocked_by_policy`
  for the policy-limited view) and discloses excluded roots / policy-hidden
  members through `excluded_roots`;
- the AI evidence handoff preserves the labeled out-of-scope / policy-limited
  rows with their omission-reason and locality tokens intact;
- the AI spend receipt's context-assembly ref equals the evidence handoff's
  scope-bound context-snapshot ref, so spend is attributed to the active scope.

The test also injects an out-of-scope context row **without** a scope label and
asserts the snapshot fails validation, so an unlabeled out-of-scope item can
never pass silently.

## Shared scope vocabulary

`manifest.json`'s `scope_class_vocabulary_map` pins the mapping between the AI
scope-class tokens and the `aureline-workspace` `ScopeClass` vocabulary. The AI
surface reuses the workspace vocabulary verbatim, so the map is the identity
over the shared classes and `ai_only_scope_classes` is empty.
`workspace_scope_class_vocabulary` mirrors `ScopeClass::as_str()`. The Rust test
reuses `aureline_workspace::ScopeClass` directly (a test-only dependency that is
acyclic — `aureline-workspace` does not depend on `aureline-ai`) and proves the
map is a bijection over the shared classes;
`ci/check_beta_ai_workset_scope.py` re-derives the workspace vocabulary from
crate source and fails closed if the map, the mirror, or the required scope-class
coverage drift, so the AI and workspace surfaces cannot fork the scope
vocabulary without breaking the gate.
