# Structural Tree Row Contract

This document freezes the shared row and placeholder contract for
structural trees: file trees, outlines, component trees, runtime DOM or
widget trees, route trees, dependency trees, write-scope preview trees,
and support/export projections. The goal is one row anatomy, one
partial-readiness vocabulary, one hidden-scope disclosure model, and one
identity/recovery model so tree surfaces do not invent local labels such
as "loading", "known", "partial", "hidden", "read-only", or "moved" in
different ways.

Machine-readable companions:

- [`/schemas/ux/tree_row.schema.json`](../../schemas/ux/tree_row.schema.json)
  defines `tree_row_record`, `tree_placeholder_record`, and
  `tree_surface_snapshot_record`.
- [`/fixtures/ux/tree_rows/`](../../fixtures/ux/tree_rows/)
  contains worked examples for partial file discovery, stale outlines,
  runtime component mapping, and moved or missing rows.

This contract composes with:

- [`/docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
  for authority, freshness, completeness, scope, and invalidation
  posture.
- [`/docs/adr/0014-search-readiness-ranking-result-truth.md`](../adr/0014-search-readiness-ranking-result-truth.md)
  and [`/docs/search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  for search readiness, hidden-scope, and partial-result language.
- [`/docs/ux/selection_and_scope_contract.md`](./selection_and_scope_contract.md)
  for focus, selection, current item, range anchor, and hidden-selected
  disclosure.
- [`/docs/ux/state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md)
  for empty, loading, degraded, paused, stale, and failed placement.
- [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
  for path identity, alias, symlink, generated artifact, and canonical
  target language.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  for generated-artifact lineage and safe-edit posture.

Where this document disagrees with those sources or the product source
documents in `.t2/docs/`, the upstream source wins and this contract,
schema, and fixtures must update in the same change.

## Scope

Frozen here:

- row anatomy for label, node kind, support or freshness badge,
  selection-sync affordance, lock or read-only state, search-match
  highlight, and hidden-scope disclosure;
- placeholder and incremental-fill behavior for discovery, hot-set
  readiness, full indexing, paused indexing, rebuilding, stale cached
  state, and failed or degraded providers;
- identity and recovery rules for moved nodes, missing nodes, generated
  artifacts, unsupported scopes, imported state, and cached state; and
- support/export fields that let diagnostics reconstruct what a tree
  claimed without raw user content or private path authority.

Out of scope:

- file-watch algorithms, virtualized tree implementation, final visual
  styling, or symbol-index quality;
- command routing for file operations, refactors, or source navigation;
  and
- replacing the search, subscription, filesystem, generated-artifact,
  or selection contracts listed above.

## One Contract Across Tree Families

Every tree surface renders rows from the same boundary model. A surface
may choose density, icons, indentation, and disclosure placement, but it
must not replace the vocabulary below with private labels.

| Surface class | Typical owner | Contract risk |
|---|---|---|
| `file_tree` | workspace runtime and VFS | Partial discovery, ignored scopes, aliases, generated files, moved or missing targets. |
| `outline_tree` | parser, language, or notebook structure provider | File-local fallback, stale symbols, unavailable provider, current-editor selection sync. |
| `component_tree` | framework analyzer, preview runtime, or language graph | Derived, imported, runtime-only, or approximate source mappings. |
| `runtime_tree` | browser, simulator, notebook, or debugger adapter | Live runtime identity, cross-origin or protocol limits, stale captured snapshots. |
| `route_tree` | framework analyzer or graph authority | Convention-derived routes, missing source, unsupported framework versions. |
| `dependency_tree` | package, graph, or provider authority | Provider limits, stale lock metadata, policy-hidden dependencies. |
| `write_scope_preview_tree` | command/review engine | Generated, read-only, excluded, blocked, or conflict rows must remain visible. |
| `support_export_tree` | support/export pipeline | Quotes tree rows and placeholders; does not mint new state. |

## Canonical Source Chain

A tree row is a materialized projection, not a second authority. Every
row names the records that produced it:

| Source | Owns | Tree row may project |
|---|---|---|
| Subscription envelope | authority class, scope, snapshot epoch, delta sequence, freshness, completeness, invalidation reason | `authority_class`, `scope_ref`, `snapshot_ref`, `freshness_class`, `completeness_class` |
| Workspace/VFS or graph identity | stable object identity, canonical target, alias or move lineage | `canonical_object_ref`, `structural_identity_ref`, `identity_state`, recovery actions |
| Structure provider | node kind, parent/child edges, expansion eligibility, provider epoch | `node_kind`, tree position, child counts, support/freshness badges |
| Search or filter session | match class, descendant match counts, hidden match reasons | `search_match_state`, match counts, hidden-scope disclosures |
| Policy/trust/provider layer | narrowed scopes, read-only or locked state, unsupported client state | `mutability_state`, hidden-scope reasons, support badges |
| Generated-artifact lineage | generator, source artifact, safe-edit posture, regeneration route | generated badges, lineage refs, recovery actions |

If a row disagrees with its source envelope, identity record, structure
provider, search packet, policy/trust layer, or generated-lineage
record, the row is wrong.

## Frozen Vocabulary

### Node Kind

`node_kind` names what the row represents, not which icon it renders.
The frozen set is:

`workspace_root`, `directory`, `file`, `generated_artifact`,
`symlink_or_alias`, `symbol`, `class`, `function`, `method`, `field`,
`module`, `package`, `route`, `component`, `service`, `runtime_node`,
`notebook_cell`, `test`, `placeholder`, `missing_target`,
`unsupported_scope`, `imported_node`, `cached_node`.

Rows may render familiar icons, but icon choice must not be the only way
the node kind is knowable.

### Readiness State

Every tree snapshot and placeholder names exactly one
`readiness_state`:

| State | Meaning | Row behavior |
|---|---|---|
| `discovering` | Discovery has started but the surface cannot yet claim the requested scope. | Render retained rows or placeholders; do not imply emptiness. |
| `hot_set_ready` | Current, recent, changed, or nearby nodes are known while the rest continues. | Hot-set rows are selectable; hidden/unknown tail remains explicit. |
| `full_indexing` | Broad discovery or structural indexing is running after useful first rows exist. | Rows stream in without losing focus, selection, or expansion identity. |
| `fully_indexed` | The requested scope is complete inside the freshness window. | This is the only state that may omit partial-readiness cues. |
| `paused` | Incremental fill is intentionally stopped by user choice, policy, resource guard, or provider backoff. | Keep known rows visible and expose resume or inspect action when allowed. |
| `rebuilding` | A prior view is being invalidated and rebuilt after schema, cache, or provider change. | Retained rows must be marked stale/rebuilding until replaced. |
| `stale` | A cached or imported view is still inspectable but below its freshness floor. | Mutating actions are blocked or revalidated; stale badge is visible. |
| `failed_degraded` | The provider failed or degraded and only a fallback or cache remains. | Show preserved capability, narrowed capability, and recovery action. |

`fully_indexed` is the only complete state. Every other state requires a
placeholder, badge, or hidden-scope disclosure that explains what is
missing, paused, stale, or degraded.

### Completeness Class

`completeness_class` is the claim a tree makes about its requested
scope:

- `complete_for_scope`
- `partial_for_scope`
- `hidden_scope_present`
- `unknown_for_scope`
- `not_applicable`

`complete_for_scope` is allowed only when the snapshot state is
`fully_indexed` and no hidden-scope disclosure exists for that scope.

### Row Badges

`row_badge_class` is a closed set. A surface may choose chip text or icon
style, but must preserve the token:

`supported`, `unsupported`, `experimental`, `cached`, `stale`,
`partial`, `imported`, `generated`, `read_only`, `policy_limited`,
`remote_limited`, `runtime_only`, `approximate_mapping`, `unknown`,
`none`.

Badges are status facts. They must not be overloaded as commands or
private provider names.

### Mutability State

`mutability_state` separates node authorship from current write ability:

`writable`, `read_only`, `locked_by_policy`, `locked_by_provider`,
`generated_read_only`, `unsupported_read_only`, `cached_read_only`,
`unknown`.

Generated and read-only are distinct. A generated row may be writable in
a regenerate flow, and a read-only row may be user-authored but blocked
by filesystem, provider, or policy state.

### Selection Sync

`selection_sync_state` names whether the row can track or reveal the
current editor/runtime/source target:

`synced_to_active_target`, `can_sync_on_focus`, `can_reveal_target`,
`out_of_scope`, `target_missing`, `mapping_approximate`, `disabled_read_only`,
`not_supported`, `unknown`.

A tree may not silently highlight a row as the current file, symbol, or
component unless it carries a stable target ref. Approximate runtime or
component mappings must remain approximate before a source jump.

### Search Match State

`search_match_state` is:

`none`, `direct_match`, `descendant_match`, `hidden_descendant_match`,
`stale_match`, `approximate_match`.

Search match highlights must be derived from a search or filter session
ref. A row with hidden matching descendants must disclose that fact even
when the descendants are outside the loaded, permitted, or visible
scope.

### Hidden Scope

A row or snapshot uses `hidden_scope_disclosure` when it represents only
part of the real object set. Reasons are:

`trust_state_excludes_surface`, `policy_narrows_source`,
`sparse_scope_excludes_root`, `outside_loaded_scope`,
`excluded_by_user_filter`, `client_scope_excludes_surface`,
`provider_overlay_unauthorised`, `remote_shard_unreachable`,
`generated_artifact_hidden_by_default`, `ignored_by_workspace_rules`,
`unsupported_scope`, `redaction_narrowed`.

Each disclosure carries a count as exact, approximate, or unknown, plus
the narrowest visible placement: inline badge, child placeholder row,
ancestor summary, or support-export-only redaction marker.

### Source and Identity

`source_state` is:

`live_discovery`, `imported_state`, `cached_snapshot`, `generated_manifest`,
`provider_overlay`, `runtime_inspection`, `support_export_snapshot`.

`identity_state` is:

`stable`, `moved_resolved`, `moved_ambiguous`, `missing_target`,
`stale_cached_identity`, `generated_artifact_lineage`,
`unsupported_scope`, `imported_cached_state`, `collision_review_required`.

Rows keep a `tree_row_id` for presentation and a separate
`structural_identity_ref` for object identity. A rename, move, remap,
refresh, placeholder replacement, or degraded-state transition must
preserve `structural_identity_ref` when the same logical object is still
known.

## Row Anatomy

Every `tree_row_record` includes:

| Field | Rule |
|---|---|
| `tree_row_id` | Stable within the snapshot and across presentational refreshes when the same projection remains. |
| `structural_identity_ref` | Stable object identity used for selection, reveal, support export, and recovery. Not a raw path. |
| `canonical_object_ref` | Optional source-owner ref for the VFS, graph, runtime, provider, generated lineage, or imported object. |
| `surface_class` | One of the tree surface classes above. |
| `parent_row_id`, `depth`, `sibling_order` | Tree placement. Reorder must not change identity. |
| `primary_label`, `secondary_label` | Redaction-aware display labels. Path or symbol text is materialized copy, not identity. |
| `label_source_class` | `canonical_owner`, `imported_metadata`, `cached_snapshot`, `runtime_observation`, `generated_lineage`, or `redacted_placeholder`. |
| `node_kind` | Closed set above. |
| `authority_class` and `source_state` | Where the row came from. |
| `freshness_class` and `completeness_class` | Whether the row is current and whether the row/scope is complete. |
| `badges[]` | Support and freshness tokens. |
| `mutability_state` | Writable, read-only, locked, generated, cached, or unknown state. |
| `selection_sync` | Current target/reveal/selection relationship and command refs. |
| `search_match` | Match/descendant/hidden match state and search-session refs. |
| `hidden_scope_disclosures[]` | Counts and reasons for omitted children, omitted descendants, or omitted peer scope. |
| `identity_recovery` | Move, missing, generated, imported, cached, or unsupported recovery posture. |

## Placeholder and Incremental Fill Rules

Placeholders are rows with a contract, not decorative loading text.
Every `tree_placeholder_record` states the scope it covers, the
readiness state, what useful content is already available, what remains
unknown, and which action is safe.

Rules:

1. `discovering` placeholders may render skeleton rows only when the
   resulting structure is predictable. Otherwise they should render a
   stable placeholder row that does not imply zero children.
2. `hot_set_ready` placeholders must keep known hot-set rows usable and
   disclose that the tail is still filling.
3. `full_indexing` placeholders may append children and refine counts,
   but must not steal focus, reset expansion, or rebind selection.
4. `paused` placeholders must say why fill paused and whether resume is
   available. A paused row is not a failure row.
5. `rebuilding` placeholders may retain stale rows, but retained rows
   must carry rebuilding or stale badges until replaced by fresh rows.
6. `stale` placeholders may expose read-only inspection and support
   export. Mutating or source-jump actions require revalidation unless a
   live owner has confirmed the target.
7. `failed_degraded` placeholders must name preserved capability,
   narrowed capability, and at least one recovery or inspect action.

Blank space is non-conforming when the surface knows that discovery is
partial, paused, rebuilding, stale, hidden, failed, or provider-limited.

## Identity and Recovery Rules

1. Selection, reveal, focus return, support export, and source jump bind
   to `structural_identity_ref`, not row label, indentation, path text,
   or current sibling order.
2. A moved row keeps identity when the owner can prove the remap. If more
   than one target matches, the row becomes `moved_ambiguous` and offers
   review rather than choosing silently.
3. A missing row remains visible when it was selected, expanded,
   recently revealed, part of a support export, or part of a pending
   action. It must carry `missing_target` plus recovery actions such as
   locate, remove from view, inspect cache, or rebuild scope.
4. Generated artifacts must carry generated lineage, safe-edit posture,
   and regeneration or source-open actions when known. They must not
   look identical to hand-authored files.
5. Unsupported scopes render an explicit unsupported row or placeholder
   with the unsupported reason and fallback route. They must not collapse
   into an empty tree.
6. Imported or cached rows remain inspectable with `imported` or
   `cached` badges and source refs. They must not claim live discovery
   until a live owner revalidates them.
7. Hidden children, hidden descendants, hidden selected rows, and hidden
   search matches remain explicit through disclosures. Counts may be
   unknown, but the omission reason cannot be omitted.

## Accessibility, Keyboard, and Export

Tree rows remain keyboard-complete:

- focus, selection, current item, and expansion state are separate;
- row actions are reachable by keyboard, even when visually deferred to
  hover/focus;
- hidden-scope, read-only, stale, generated, unsupported, and degraded
  cues have accessible labels and are not color-only;
- placeholder rows are focusable when they contain an action or explain
  why rows are missing; and
- support/export packets preserve token values, identity refs, hidden
  counts, and recovery posture with redaction-aware labels.

## Non-Conforming Patterns

- An empty file tree while discovery is still running.
- A full-looking outline when only file-local symbols are available.
- A component tree that jumps to source from an approximate or
  runtime-only node without disclosing mapping quality.
- A generated file row that uses the same writable state as a
  hand-authored file.
- A moved or missing row that disappears while selected, expanded, or
  referenced by support export.
- A hidden-policy, ignored-scope, or provider-limited subtree represented
  only by absence.

## Required Fixture Coverage

The fixture corpus covers:

- hot-set file discovery with hidden scope and incremental fill;
- stale cached outline rows with rebuilding placeholders;
- component/runtime rows with partial or approximate source mapping; and
- moved, missing, generated, imported, cached, and unsupported recovery
  states.
