# Topology propagation across search, AI context, and mutation review

Repository topology only stays honest if every surface that talks about repository
truth reads the *same* boundary. The topology descriptors and their surface
bindings record why an answer is partial; this contract propagates that truth into
the three remaining first consumers — **search scope**, **AI context assembly**,
and **mutation review** — so a topology gap is never silently flattened into "no
results", "the file is empty", or "apply to everything".

Every packet here is derived from the canonical `git_topology_surface_binding`
projections (`TopologyRootDescriptor::project`) or the descriptors themselves, so
the surfaces cannot drift: there is one topology truth, not a search copy, an AI
copy, and a review copy.

- Search schema: [`schemas/git/search_topology_scope.schema.json`](../../../schemas/git/search_topology_scope.schema.json)
- AI schema: [`schemas/git/ai_topology_context.schema.json`](../../../schemas/git/ai_topology_context.schema.json)
- Review overlay schema: [`schemas/git/review_topology_overlay.schema.json`](../../../schemas/git/review_topology_overlay.schema.json)
- Canonical packets: [`artifacts/git/m5/git_topology/topology_propagation/`](../../../artifacts/git/m5/git_topology/topology_propagation)
- Fixtures: [`fixtures/git/m5/topology-propagation/`](../../../fixtures/git/m5/topology-propagation)
- Code: `crates/aureline-search/src/topology_propagation/`, `crates/aureline-ai/src/topology_context/`, `crates/aureline-review/src/topology_overlays/`

## Search scope

Each `search_topology_scope_row` is derived from one `search_scope` surface
binding. It carries the explicit topology result truth and, when one exists, the
reviewed remediation verb (`widen` / `deepen` / `initialize` / `hydrate`) — the
same verbs the action sheets and review lanes surface.

The decisive invariant is `zero_results_means_absent`: it is true only when the
result truth is `complete`. For an omitted slice, unfetched objects, an
uninitialized submodule, pointer-only assets, a generated/vendor root, or a
wrong-target/nested root, search reports the explicit limit and keeps the owning
`authoritative_root_ref` visible — it never asserts that a topology gap means the
content is absent, and a wrong-root row recommends retargeting rather than widening
the wrong root.

## AI context

Each `ai_topology_context_slice_row` is derived from one `ai_context` surface
binding. Two flags protect the model:

- `content_is_authoritative` / `admit_body_to_prompt` are true only for complete,
  in-scope, hydrated slices. A pointer-only or unfetched slice is named, not pasted
  into the prompt as if it were the file.
- `crosses_repo_boundary` stays visible whenever the slice belongs to a different
  root than the active one, so a parent and a child repository are never folded
  into one undifferentiated context.

Each limited slice carries the same reviewed remediation verb the other surfaces
offer.

## Mutation-review overlay and the multi-root guard

Each `review_topology_overlay_root_row` is the deterministic `review`-surface
projection of one descriptor, so parent/child repo identity (`identity_kind`,
`parent_root_ref`) and worktree/root identity stay visible during mutation review,
and a non-active root is never mutable in the active ambient action.

The `review_multi_root_mutation_preview` is the guard. When the proposed mutation
set touches more than one root it sets `spans_multiple_roots`, requires the
`explicit_multi_root_preview_required` scope, and marks `auto_apply_blocked` and
`opt_in_required` true. It also reports whether the set crosses a parent/submodule
boundary or a nested-independent-repo boundary. Cross-root bulk mutation stays
preview-first and opt-in; a single active root needs no cross-root preview.

## Invariants

- The three surfaces share one topology vocabulary; rows and previews are the
  deterministic derivation of the canonical bindings/descriptors and are
  re-validated against them.
- Search never treats omitted or unfetched content as absent
  (`zero_results_means_absent` only when complete).
- AI never admits a topology-limited slice as authoritative prompt material and
  never crosses a repo boundary silently.
- Review never mutates a non-active root in the ambient action, and a cross-root
  mutation set is always guarded preview-first and opt-in.
- The remediation verb mapping matches `aureline_git::topology_actions`, so all
  surfaces recommend the same widen/deepen/initialize/hydrate verb.
- Every support export retains its reconstruction fields after redaction and
  asserts that raw paths and object bytes are redacted.

Regenerate the canonical packets and fixtures with:

```bash
cargo run -p aureline-search --example dump_search_topology_scope \
  > artifacts/git/m5/git_topology/topology_propagation/search_topology_scope.json
cargo run -p aureline-search --example dump_search_topology_scope -- cross-root \
  > fixtures/git/m5/topology-propagation/search_cross_root_wrong_target.json
cargo run -p aureline-ai --example dump_ai_topology_context \
  > artifacts/git/m5/git_topology/topology_propagation/ai_topology_context.json
cargo run -p aureline-ai --example dump_ai_topology_context -- cross-root \
  > fixtures/git/m5/topology-propagation/ai_cross_root_boundary.json
cargo run -p aureline-review --example dump_review_topology_overlay \
  > artifacts/git/m5/git_topology/topology_propagation/review_topology_overlay.json
cargo run -p aureline-review --example dump_review_topology_overlay -- single \
  > fixtures/git/m5/topology-propagation/review_single_root_allowed.json
```
