# Collection-view, filter-AST, saved-view, and batch-review contract

This document is the **product-wide contract** for dense collection
behaviour every search, review, log, package, work-item, admin, and
support surface inherits when it filters, sorts, virtualises, saves,
exports, or runs a batch action over a typed item population. It
freezes one filter AST, one saved-view artifact shape, one batch-
review packet shape, one count and truth vocabulary, and one set of
identity / scope / fallback rules so collection-bearing surfaces
cannot mint local synonyms for `visible`, `loaded`, `matching`,
`selected`, `approx.`, `exact`, `provider-limited`, `stale`,
`cached`, or `partial`.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream surface's mint of its own
filter, saved view, or batch sheet, this document wins and the
surface is non-conforming.

The companion artifacts are:

- [`/schemas/collections/filter_ast.schema.json`](../../schemas/collections/filter_ast.schema.json)
  — boundary schema for the typed, serialisable filter AST every
  collection surface uses to express, persist, and re-resolve
  filters across providers, saved views, deep links, exports, and
  CLI output.
- [`/schemas/collections/saved_view.schema.json`](../../schemas/collections/saved_view.schema.json)
  — boundary schema for the durable saved-view artifact (owner
  class, scope, privacy class, schema version, column set, sort
  order, group-by, fallback behaviour) every collection surface
  reads when restoring or sharing a view.
- [`/schemas/collections/batch_review_packet.schema.json`](../../schemas/collections/batch_review_packet.schema.json)
  — boundary schema for the batch-review packet emitted before
  every destructive, provider-owned, remote, or export-bearing
  batch action.
- [`/fixtures/collections/batch_review_examples/`](../../fixtures/collections/batch_review_examples/)
  — worked-example corpus covering visible-only batches, escalation
  to all-matching, provider-limited approximations, blocked /
  unavailable members, and stale-snapshot recovery.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — `interaction_safety_packet_record`, `batch_scope_record`,
  representation class, focus-return state, responsive fallback,
  preview / apply / revert phases, denial vocabulary. Every batch-
  review packet in this contract pairs with exactly one
  `interaction_safety_packet_record` whose `batch_scope` quotes the
  same scope class; this contract names the *population* that
  packet operates on, not a parallel safety vocabulary.
- [`/docs/ux/live_update_review_contract.md`](./live_update_review_contract.md)
  — `live_set_state_record`, review-control state, delivery state,
  authority-limit state, count truth (`loaded`, `visible`,
  `total`), buffered-change indicators, anchor state, batch-
  membership honesty (`stable`, `drifting`, `approximate`,
  `snapshot_pinned`), schema drift, copy / export posture. This
  contract names the *durable filter / saved view / batch-review
  packet* a live surface reads from; the live contract names the
  *moment-to-moment liveness* of the underlying stream. The two
  compose: a saved view resolved against a `live` surface inherits
  the live record's drift, anchor, and provider-limit state.
- [`/docs/navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md)
  — saved-query bundle, search-collection-snapshot, reopen-honesty
  state, sensitive-literal handling, share-policy, and provider-
  backing classes. A saved view that wraps a saved query cites the
  saved query by opaque ref and inherits the bundle's privacy and
  share posture; this contract MUST NOT redeclare the saved-query
  registry shape.
- [`/docs/verification/focus_and_batch_scope_packet.md`](../verification/focus_and_batch_scope_packet.md)
  and
  [`/fixtures/ux/selection_and_virtualization_manifest.yaml`](../../fixtures/ux/selection_and_virtualization_manifest.yaml)
  — selection-scope class, count-term vocabulary, focus-return
  state, range-anchor state, hidden-selected disclosure, and
  blocked-versus-skipped separation for virtualised tables and
  lists. Every batch-review packet in this contract resolves a
  `selection_scope_class` from that manifest.
- [`/docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
  — authority class, scope, freshness, completeness, and
  invalidation posture on the underlying subscription stream. Every
  saved view and batch-review packet pins a `freshness_class` from
  that ADR.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — broker-owned redaction pass. Filter-AST literals, saved-view
  labels, and batch-review packets carry opaque refs and reviewable
  labels only; raw literals never cross any of the three boundary
  schemas.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `redaction_class`
  re-exported without modification.
- [`/docs/adr/0014-search-readiness-ranking-result-truth.md`](../adr/0014-search-readiness-ranking-result-truth.md)
  — `surface_class`, `result_truth_class`, `scope_filter_class`,
  `partial_truth_cause`, `hidden_scope_reason`. Saved views that
  ride a search surface resolve their hidden-row counts through
  that ADR's vocabulary.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does not ship a collection UI implementation, a saved-
view share UI, a batch-review sheet engine, a filter-builder
component, or a CLI rendering pipeline. It freezes the artifact
shapes those implementations will read and write. The eventual
collection / shell crate's Rust types are the schema of record; the
JSON Schema exports at
`schemas/collections/filter_ast.schema.json`,
`schemas/collections/saved_view.schema.json`, and
`schemas/collections/batch_review_packet.schema.json` are the
cross-tool boundaries every non-owning surface reads.

## Why freeze this now

Without one frozen contract every dense collection surface is free
to invent its own per-feature notion of "filter", "saved view",
"select all", "matching", "blocked", "stale snapshot", and "what
the batch action will actually touch". Each divergence widens a
different axis silently:

1. *Search and review use incompatible filter ASTs, so a saved
   view in one cannot resolve in the other.* Reviewers and authors
   lose cross-surface continuity because each surface owns its own
   syntax tree.
2. *A saved view silently resurrects against a provider that no
   longer admits one of its filter operators.* The user sees the
   wrong rows and cannot tell that the provider quietly dropped a
   constraint.
3. *A batch-review sheet says "Delete 432 items" while the visible
   set is 80 and the matching set against the provider is 4,310.*
   The user commits to a wider population than the sheet
   represented.
4. *Selection identities silently rebind after a sort change so a
   later batch action targets different objects than the user
   approved.* Filtering, sorting, or virtualisation widens the
   blast radius without the user ever seeing it.
5. *Sensitive filter literals (file-name fragments, secret stems,
   identifier prefixes) leak into shared infrastructure when a
   saved view auto-shares to an org index.* The local-only default
   was never honoured.
6. *Provider-limited counts paint as exact totals on export and
   CLI output.* Approximations the surface knew about disappear
   the moment bytes leave the product.

The freeze matters now, ahead of any collection UI landing, so
every later surface can read **the same** filter AST, **the same**
saved-view artifact, **the same** batch-review packet, **the same**
count and truth vocabulary, and **the same** identity / scope /
fallback rules instead of inventing per-surface equivalents.

## Who reads this document

- **Surface authors** building dense tables, result grids, log
  viewers, package inventories, work-item lists, admin
  configuration grids, and review collections.
- **Product writers** choosing copy for `Select all visible`,
  `Select all matching`, count chips, blocked / unavailable banners,
  stale-snapshot disclosures, and saved-view restore prompts.
- **Support and parity-audit tooling** that needs one machine-
  readable packet explaining what the user reviewed, what was
  excluded, and what the batch action ultimately targeted.
- **Extension and CLI authors** that emit, consume, or re-export
  collection state across surface boundaries.

## One contract, six surface families, three artifact shapes

The contract applies uniformly to the surface families below. A
surface that mints a private filter syntax, a private saved-view
shape, a private batch-review sheet, or a private count vocabulary
is non-conforming.

| Surface family               | Typical examples                                                                                       | Review risk to control                                                                                                                       |
|------------------------------|--------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| `search_collection`          | code search, symbol search, docs search, multi-repo result grid                                        | hidden rows from policy / provider narrowing, deep-link reopen drift, sampled / capped totals                                                 |
| `review_collection`          | review queue, batched diff list, fix-suggestion inbox, AI-evidence rollup                              | apply-basis drift across the reviewed batch, hidden blocked members, partial-apply / partial-revert                                           |
| `log_or_event_collection`    | task logs, container logs, pipeline runs, activity feed, incident timeline                             | retention windows, provider truncation, time-range ambiguity, autoscroll vs frozen review                                                     |
| `package_or_inventory_grid`  | extension inventory, dependency list, asset registry, release artefact list                            | publisher trust, version drift, provider-owned read-only rows, ownership-blocked deletes                                                      |
| `work_item_collection`       | issue list, task board, runbook checklist, release-train queue                                         | scope filter drift, role-based visibility, ownership-blocked moves, cross-workspace identity                                                   |
| `admin_or_settings_grid`     | identity / role list, policy / lock list, audit-event grid, managed-workspace inventory                | policy-narrowing changes, provider-owned writes, evidence-only rows, change-window approvals                                                   |

Every surface above — and every future surface that inherits the
same review risk — emits

1. a `filter_ast_record` whenever a filter is persisted, exported,
   shared, or re-resolved across a surface boundary;
2. a `saved_view_record` whenever a column set, filter, sort
   order, or group-by is saved, restored, or shared;
3. a `batch_review_packet_record` before every destructive,
   provider-owned, remote, or export-bearing batch action — paired
   to exactly one `interaction_safety_packet_record` whose
   `batch_scope_record` quotes the same scope class.

## Frozen vocabulary

This contract introduces the following frozen vocabularies. Each
is owned by exactly one of the three boundary schemas; downstream
surfaces re-export by reference and never mint a parallel value.

### Count-term vocabulary (frozen, ten values)

`collection_count_term` (owned by the batch-review packet schema,
re-used by the saved-view schema):

- `visible` — rows currently rendered in the reviewed view.
- `loaded` — rows materialised in client memory or pinned by the
  provider cursor.
- `matching` — rows the current filter AST resolves to against the
  authoritative source (server-side when the source is provider-
  authoritative; client-side when local).
- `selected` — rows the user has explicitly admitted to the current
  selection.
- `included` — selected rows the next batch action will actually
  attempt to act on.
- `excluded` — selected rows excluded by a user-applied filter
  refinement at commit time.
- `blocked` — rows the next action cannot run on (policy, ownership,
  protected path, missing permission, freshness gate).
- `unavailable` — rows the surface cannot resolve right now
  (provider offline, filter unsupported by provider, column
  redacted, authority class too low).
- `skipped` — rows the action chose not to mutate post-commit
  because they were already compliant or no-op.
- `hidden_selected` — selected rows currently outside the visible
  filter / window but still members of the selection.
- `not_loaded` — selected rows whose identity is known but whose
  body is not currently materialised on the client.

`hidden_selected` and `not_loaded` are intentionally separate axes;
collapsing them is non-conforming.

### Count-status vocabulary (frozen, seven values)

`collection_count_status` (owned by the batch-review packet schema):

- `exact` — the producer can claim the count holds exactly under
  the current basis.
- `approximate` — the producer admits a bounded approximation and
  carries a numeric value plus a tolerance label.
- `provider_limited` — the upstream provider capped or sampled the
  count; the value is whatever the provider exposes and MUST NOT
  be flattened to `exact`.
- `stale` — the producer can no longer claim continuity with the
  authoritative basis; the count is the last known value.
- `cached` — the count is from a warm cache and not re-confirmed
  against authority since the cache was warmed.
- `partial` — the producer ran out of budget / time / page-cursor
  pages before a full count completed.
- `unknown` — the producer cannot claim a count at all and refuses
  to invent one.

The vocabulary travels with every `selected`, `visible`, `loaded`,
`matching`, `included`, `excluded`, `blocked`, `unavailable`,
`skipped`, `hidden_selected`, and `not_loaded` figure. A surface
that emits a count without a status is non-conforming.

### Filter-AST operator vocabulary (frozen, twenty-five values)

`filter_operator_class` (owned by the filter-AST schema):

Combinators (closed):

- `combinator_all_of`
- `combinator_any_of`
- `combinator_none_of`
- `combinator_not`

Equality and set membership (closed):

- `equals`
- `not_equals`
- `in_set`
- `not_in_set`

Substring and pattern (closed; pattern dialect named separately):

- `contains_substring`
- `does_not_contain_substring`
- `matches_pattern`
- `does_not_match_pattern`

Numeric and ordinal (closed):

- `greater_than`
- `greater_than_or_equal`
- `less_than`
- `less_than_or_equal`
- `within_range`
- `outside_range`

Time (closed):

- `before_time`
- `after_time`
- `within_time_window`

Nullability (closed):

- `is_null`
- `is_not_null`

Tag and scope (closed):

- `tag_includes`
- `tag_excludes`

Pattern dialect (named separately so providers without regex still
type-check the AST):

- `pattern_dialect_glob`
- `pattern_dialect_extended_regex`
- `pattern_dialect_literal_segment`

A node whose operator is `matches_pattern` or
`does_not_match_pattern` MUST name exactly one
`pattern_dialect_*` value; a missing dialect denies with
`filter_pattern_dialect_unspecified`.

### Filter-source classes (frozen, six values)

`filter_source_class`:

- `client_local_filter` — every operator and field resolves on the
  client; the provider receives the unfiltered set or no query at
  all.
- `provider_authoritative_filter` — the provider applies the
  filter and returns the matching set; the client renders provider
  truth.
- `provider_translated_filter` — the client translates the AST into
  the provider's native language and the provider applies the
  result; the surface MUST disclose translation losses.
- `policy_narrowed_filter` — an admin policy narrows the user's
  AST before resolution; the surface renders the narrowing as a
  pinned, read-only subtree.
- `saved_view_pinned_filter` — the AST is pinned by a saved view
  and is read-only on consumer surfaces.
- `composed_local_over_provider` — the provider applies a coarse
  match and the client refines locally; counts MUST distinguish the
  provider-side and client-side populations.

### Filter-field source classes (frozen, five values)

`filter_field_source_class`:

- `surface_native_field` — first-class field on the surface's row.
- `derived_field_local` — computed locally from native fields.
- `derived_field_provider` — computed by the provider against
  provider-owned data.
- `provider_only_field` — exists only on the provider; cannot be
  resolved locally.
- `policy_redacted_field` — present in the type system but masked
  by policy on the current viewer.

A `provider_only_field` referenced under
`client_local_filter` denies with
`filter_field_unresolvable_in_local_source`.

### Filter-literal handling classes (frozen, five values)

`filter_literal_handling_class`:

- `literal_redacted_at_boundary_default`
- `literal_hashed_with_local_salt`
- `literal_kept_in_local_only_store`
- `literal_kept_in_org_governed_vault`
- `literal_disclosed_with_explicit_user_opt_in`

Re-exported verbatim from
`schemas/search/saved_query_bundle.schema.json` so saved views and
saved queries share one literal-handling story. The default for
every filter literal that crosses the schema boundary is
`literal_redacted_at_boundary_default`; the raw literal lives in
the local store only and never leaves it.

### Filter compatibility classes (frozen, four values)

`filter_compatibility_class`:

- `compatible_current` — every operator, field, and dialect
  resolves under the current build's filter resolver.
- `forward_compatible_unknown_operator_isolated` — the AST contains
  an operator the current build does not know, and the unknown
  subtree is isolated (the AST resolver returns the surrounding
  combinator's deny-closed default for that subtree without
  silently dropping it).
- `incompatible_unknown_operator_blocking` — an unknown operator
  appears outside an isolatable subtree and the AST refuses to
  resolve.
- `incompatible_unknown_field_blocking` — an unknown field appears
  outside an isolatable subtree and the AST refuses to resolve.

The schema gates pair every node to its compatibility class; an
AST that asserts `compatible_current` while carrying an unknown
operator denies with `filter_compatibility_class_required_action_mismatch`.

### Saved-view owner classes (frozen, eight values)

`saved_view_owner_class` (owned by the saved-view schema):

- `user_owned_local`
- `user_owned_portable_export`
- `workspace_committed_shared`
- `org_curated_admin_writable`
- `org_published_admin_read_only`
- `policy_pinned_admin_locked`
- `provider_owned_read_only`
- `support_export_captured`

### Saved-view scope classes (frozen, five values, spec-aligned)

`saved_view_scope_class`:

- `user`
- `workspace`
- `shared`
- `policy_pinned`
- `provider_owned`

The five values match the deliverable list verbatim. They compose
with `saved_view_owner_class`: a `policy_pinned` scope MUST resolve
to either `policy_pinned_admin_locked` or
`provider_owned_read_only`; a `provider_owned` scope MUST resolve
to `provider_owned_read_only`.

### Saved-view privacy classes (frozen, four values)

`saved_view_privacy_class` re-exported verbatim from the
capability-lifecycle redaction class:

- `metadata_safe_default`
- `operator_only_restricted`
- `internal_support_restricted`
- `signing_evidence_only`

### Saved-view fallback classes (frozen, seven values)

`saved_view_fallback_class`:

- `degrade_to_local_subset_with_disclosure`
- `pin_filter_subtree_disabled_with_disclosure`
- `pin_column_unavailable_with_placeholder`
- `pin_view_unresolvable_offer_recreate`
- `pin_view_archived_offer_restore`
- `pin_view_unavailable_provider_offline_disclosed`
- `pin_view_unavailable_policy_narrowed_disclosed`

The required degraded behaviour: a saved view whose provider lane,
filter operator, field, or column became unavailable since capture
MUST resolve to exactly one fallback class and disclose it on the
restored surface; silently wiping user state or pretending the view
restored is non-conforming.

### Saved-view drift states (frozen, seven values)

`saved_view_drift_state`:

- `bound_current_state_matches_captured`
- `provider_state_drifted_disclosed`
- `column_set_drifted_disclosed`
- `policy_narrowing_changed_disclosed`
- `view_archived_offered_restore`
- `view_unresolvable_offered_recreate`
- `view_unavailable_provider_offline_disclosed`

A saved view that resolves silently as `bound_current_state_matches_captured`
while the provider, columns, or policy narrowing changed denies
with `saved_view_drift_state_required_action_mismatch`.

### Batch-execution-origin classes (frozen, three values)

`batch_execution_origin_class`:

- `client_local_execution` — the action runs entirely on the
  client; no provider call.
- `provider_authoritative_execution` — the action runs on the
  provider; the client has no local-only path.
- `mixed_client_then_provider` — the client commits a local pass
  then delegates the remainder to the provider; counts MUST split
  on the seam.

### Batch scope-truth classes (frozen, three values)

`batch_scope_truth_class`:

- `client_local_truth` — the reviewed population is the client's
  responsibility; the server has no opinion.
- `provider_authoritative_truth` — the reviewed population is the
  provider's authoritative set; the client may render only a
  visible window.
- `mixed_truth_resolved_by_provider` — the client built the
  selection but the provider re-resolves at apply time; the client
  MUST disclose the re-resolution before commit.

### Reviewed-population basis classes (frozen, eight values)

`reviewed_population_basis_class`:

- `current_visible_set`
- `current_filter_sort`
- `loaded_window`
- `query_snapshot`
- `time_window`
- `provider_cursor_window`
- `snapshot_basis`
- `explicit_custom_set`

### Selection-scope classes (frozen, five values)

`selection_scope_class` re-exported verbatim from
`fixtures/ux/selection_and_virtualization_manifest.yaml`:

- `current_item_only`
- `visible_range`
- `loaded_set`
- `all_matching_query`
- `explicit_custom_set`

### Item-identity stability classes (frozen, three values)

`batch_item_identity_class`:

- `stable_item_identity` — the surface owns a stable client-side
  id that survives sort, filter, and virtualisation; later actions
  target the exact admitted set.
- `provider_owned_identity_pinned` — identity comes from the
  provider; the surface pins the provider id and MUST refuse a
  later action whose pinned id no longer resolves.
- `local_alias_identity_disclosed` — the surface uses a local
  alias because the provider id is unavailable; the alias is
  disclosed and a later action denies on cross-session use.

### Blocked reasons (frozen, seven values)

`batch_blocked_reason_class`:

- `blocked_by_policy`
- `blocked_by_ownership`
- `blocked_by_protected_path`
- `blocked_by_provider_unsupported`
- `blocked_by_freshness_required`
- `blocked_by_grant_missing`
- `blocked_by_concurrent_edit`

### Unavailable reasons (frozen, four values)

`batch_unavailable_reason_class`:

- `unavailable_provider_offline`
- `unavailable_filter_unsupported_by_provider`
- `unavailable_column_redacted`
- `unavailable_authority_class_lower_than_required`

### Recovery-guidance classes (frozen, six values)

`batch_recovery_guidance_class` re-exported from the interaction-
safety contract:

- `inline_undo_revert_available`
- `compensating_action_only`
- `regenerate_from_source`
- `evidence_only_no_rerun`
- `restore_from_checkpoint`
- `no_recovery_available`

### Denial-reason vocabularies (frozen)

Filter AST: `filter_ast_unknown`,
`filter_ast_node_unknown`,
`filter_pattern_dialect_unspecified`,
`filter_field_unresolvable_in_local_source`,
`filter_compatibility_class_required_action_mismatch`,
`filter_literal_handling_class_required_action_mismatch`,
`filter_literal_export_requires_explicit_opt_in`,
`raw_body_forbidden_on_boundary`,
`filter_ast_schema_version_lagging`,
`policy_epoch_expired`,
`policy_blocked`.

Saved view: `saved_view_unknown`,
`saved_view_owner_scope_mismatch`,
`saved_view_privacy_class_mismatch_for_owner`,
`saved_view_drift_state_required_action_mismatch`,
`saved_view_fallback_class_required_for_drift`,
`saved_view_pinned_filter_resolution_required`,
`saved_view_column_set_drift_undisclosed`,
`saved_view_share_disabled_by_policy`,
`raw_body_forbidden_on_boundary`,
`saved_view_schema_version_lagging`,
`policy_epoch_expired`,
`policy_blocked`.

Batch-review packet: `batch_review_packet_unknown`,
`batch_population_basis_unknown`,
`batch_scope_truth_class_unlabelled`,
`batch_execution_origin_unlabelled`,
`batch_count_status_missing`,
`batch_count_term_collapsed`,
`batch_blocked_reason_unlabelled`,
`batch_unavailable_reason_unlabelled`,
`batch_recovery_guidance_missing`,
`batch_item_identity_class_unlabelled`,
`batch_scope_widened_post_review`,
`batch_provider_authoritative_count_missing_required`,
`raw_body_forbidden_on_boundary`,
`batch_review_packet_schema_version_lagging`,
`policy_epoch_expired`,
`policy_blocked`.

## Truthfulness posture (normative)

Every rule below is normative. A new collection-bearing surface
that violates any of them is non-conforming regardless of how the
violation is painted.

1. **One filter AST.** Every persisted, exported, shared, or
   cross-surface filter is a `filter_ast_record`. Search, review,
   log, package, work-item, and admin surfaces all read this AST
   shape; cross-surface restore and CLI output reference the same
   tree. A surface that mints a private filter syntax denies on
   re-resolution with `filter_ast_node_unknown`.
2. **One saved-view artifact.** Every saved column set, filter,
   sort, group-by, or column-width preset is a `saved_view_record`.
   The owner class, scope class, privacy class, and fallback class
   are separately addressable; collapsing them into one chip is a
   UI freedom but the boundary record MUST keep them addressable.
3. **One batch-review packet per consequence-bearing batch.** A
   `batch_review_packet_record` is emitted before every
   destructive, provider-owned, remote, or export-bearing batch
   action and pairs to exactly one
   `interaction_safety_packet_record`. Skipping the packet is non-
   conforming.
4. **Counts carry status.** Every `selected`, `visible`, `loaded`,
   `matching`, `included`, `excluded`, `blocked`, `unavailable`,
   `skipped`, `hidden_selected`, and `not_loaded` figure carries a
   `collection_count_status`. A surface that renders a number
   without a status denies with `batch_count_status_missing`.
5. **Approximations stay approximate.** A surface MAY NOT promote
   `approximate`, `provider_limited`, `stale`, `cached`, `partial`,
   or `unknown` to `exact` on copy / export, deep-link, CLI output,
   support capture, or batch-review preview. Provider-limited
   counts on a destructive review remain provider-limited.
6. **Visible, loaded, and matching are separate axes.** A
   collection MAY NOT collapse two of these into one number. The
   batch-review packet MUST quote each axis with its own count
   value and status; the saved view MUST pin the axes the user
   captured against.
7. **Selected vs hidden-selected vs not-loaded stay separate.** A
   surface that flattens `hidden_selected` or `not_loaded` into
   `selected` denies with `batch_count_term_collapsed`.
   `hidden_selected` and `not_loaded` are inspectable via keyboard
   on every dense surface (this contract composes with the
   selection-and-virtualisation manifest).
8. **Blocked and unavailable are not skipped.** Blocked rows are
   refused before commit; unavailable rows are absent from
   evaluation; skipped rows are post-commit no-ops. Collapsing them
   denies with `batch_blocked_reason_unlabelled`,
   `batch_unavailable_reason_unlabelled`, or
   `batch_count_term_collapsed`.
9. **Item identity survives sort, filter, and virtualisation.**
   Every batch packet pins
   `batch_item_identity_class` and the included id refs. A surface
   that allows sort / filter / virtualisation to silently change
   which objects a later action targets denies with
   `batch_scope_widened_post_review`.
10. **Saved views degrade visibly.** A saved view whose provider
    lane, filter operator, field, column, or policy narrowing
    drifted MUST resolve to exactly one
    `saved_view_drift_state` and exactly one
    `saved_view_fallback_class`. Silently wiping user state or
    rendering the view as bound denies with
    `saved_view_drift_state_required_action_mismatch`.
11. **Sensitive filter literals are local-only or hashed by
    default.** Every filter that crosses the schema boundary
    defaults to `literal_redacted_at_boundary_default`. A literal-
    disclosing class without explicit user opt-in denies with
    `filter_literal_export_requires_explicit_opt_in`.
12. **Provider-authoritative truth is not optional on
    destructive batches.** A `provider_authoritative_execution` or
    `mixed_client_then_provider` batch MUST quote the provider's
    matching count with status (`exact`, `approximate`,
    `provider_limited`, `stale`, `cached`, `partial`, `unknown`).
    A missing provider matching count denies with
    `batch_provider_authoritative_count_missing_required`.
13. **No raw payloads cross any boundary.** Raw absolute paths,
    raw symbol bodies, raw document bodies, raw row text, raw URLs,
    raw filter literal bytes, and raw query bodies MUST NOT appear
    on any record emitted against any of the three schemas. A row
    that carries a raw payload denies with
    `raw_body_forbidden_on_boundary`.

## Filter AST (normative)

The filter AST is a typed tree. Every node names exactly one
`filter_operator_class` and every leaf node names exactly one
`filter_field_source_class` and one `filter_literal_handling_class`.

### Tree shape

- A combinator node (`combinator_all_of`, `combinator_any_of`,
  `combinator_none_of`, `combinator_not`) has zero or more child
  nodes. `combinator_not` MUST have exactly one child.
- A leaf node names a `field_ref` (opaque ref to the field
  registry; raw field names never cross the boundary), an operator
  class, and operator-appropriate operands (`literal_value_ref`,
  `value_set_ref`, `range_ref`, `time_window_ref`, `tag_ref`).
- Every node carries `filter_compatibility_class`,
  `filter_source_class`, and `unknown_node_isolation_state`
  (`isolated`, `not_isolatable`).
- Pattern operators carry exactly one `pattern_dialect_class`.

### Cross-surface re-resolution

When a filter AST is restored on a different build, surface, or
provider, the resolver:

1. type-checks every node against the current field registry;
2. resolves every operator against the current operator support
   matrix;
3. resolves every dialect against the current pattern engine;
4. emits a top-level `filter_compatibility_class`;
5. emits per-node compatibility classes;
6. emits any `unknown_node_isolation_state`.

A re-resolution that returns `compatible_current` while any node
carries an unknown operator or unknown field denies with
`filter_compatibility_class_required_action_mismatch`.

### Privacy and redaction

Every literal carries:

- `filter_literal_handling_class`;
- `literal_value_ref` (opaque ref into the local-only literal
  store; raw literal bytes never cross);
- `display_label_opaque_ref` (optional opaque ref into the
  reviewable-label registry; raw label bytes never cross);
- `policy_context` (re-exported).

The default handling for every literal that crosses the schema
boundary is `literal_redacted_at_boundary_default`.

### Compatibility versioning

The schema carries a top-level
`filter_ast_schema_version` integer. Adding a new operator,
dialect, source class, field source class, or compatibility class
is additive-minor and bumps the version; repurposing an existing
member is breaking and requires a new decision row in
`artifacts/governance/decision_index.yaml`.

A consumer that reads a version higher than its own resolver
supports denies with `filter_ast_schema_version_lagging` and falls
back to the saved view's
`pin_view_unresolvable_offer_recreate` fallback.

## Saved-view artifact (normative)

Every saved view carries:

- `saved_view_id` (opaque);
- `saved_view_owner_class`;
- `saved_view_scope_class`;
- `saved_view_privacy_class`;
- `saved_view_schema_version`;
- `surface_class` (re-exported);
- `filter_ast_id_ref` (opaque ref to the captured filter AST);
- `column_set_ref` (opaque ref to the captured column set);
- `sort_order_refs` (opaque refs to the captured sort directives);
- `group_by_ref` (optional);
- `pinned_count_axes` (subset of
  `visible`, `loaded`, `matching`,
  `selected`, `provider_side_query_scope`);
- `saved_view_fallback_class` (default fallback when restore
  cannot bind exactly);
- `running_build_identity_ref`;
- `policy_context`;
- `client_scopes`;
- `redaction_class`;
- `freshness_class`;
- `created_at`, `updated_at`, `archived_at`, `tombstoned_at`.

### Owner / scope / privacy gating

The schema enforces:

- `user_owned_local` resolves to scope `user` and privacy
  `metadata_safe_default`.
- `workspace_committed_shared` resolves to scope `workspace` or
  `shared`.
- `org_curated_admin_writable` and
  `org_published_admin_read_only` resolve to scope `shared`.
- `policy_pinned_admin_locked` resolves to scope
  `policy_pinned`.
- `provider_owned_read_only` resolves to scope
  `provider_owned`.
- `support_export_captured` resolves to scope `user` or
  `shared` and privacy
  `internal_support_restricted` or
  `signing_evidence_only`.

### Fallback when restore cannot bind exactly

A saved view restore resolves exactly one
`saved_view_drift_state` and pairs it to exactly one
`saved_view_fallback_class`:

| Drift                                            | Required fallback                                                        |
|--------------------------------------------------|--------------------------------------------------------------------------|
| `bound_current_state_matches_captured`           | none required.                                                            |
| `provider_state_drifted_disclosed`               | `pin_filter_subtree_disabled_with_disclosure` or `degrade_to_local_subset_with_disclosure`. |
| `column_set_drifted_disclosed`                   | `pin_column_unavailable_with_placeholder`.                                |
| `policy_narrowing_changed_disclosed`             | `pin_view_unavailable_policy_narrowed_disclosed`.                         |
| `view_archived_offered_restore`                  | `pin_view_archived_offer_restore`.                                        |
| `view_unresolvable_offered_recreate`             | `pin_view_unresolvable_offer_recreate`.                                   |
| `view_unavailable_provider_offline_disclosed`    | `pin_view_unavailable_provider_offline_disclosed`.                        |

A drifted view rendered as bound denies with
`saved_view_drift_state_required_action_mismatch`. A drifted view
without a fallback class denies with
`saved_view_fallback_class_required_for_drift`.

## Batch-review packet (normative)

A batch-review packet is the population side of the
`interaction_safety_packet_record`. Every destructive, provider-
owned, remote, or export-bearing batch action emits exactly one
packet before commit. The packet carries:

- `batch_review_packet_id` (opaque);
- `paired_interaction_safety_packet_id_ref` (opaque ref to the
  paired interaction-safety packet);
- `surface_class`;
- `reviewed_population_basis_class`;
- `selection_scope_class`;
- `batch_execution_origin_class`;
- `batch_scope_truth_class`;
- `batch_item_identity_class`;
- `included_item_id_refs` (opaque, ordered, unique);
- `excluded_item_id_refs` (opaque);
- `blocked_item_summary` (per-reason counts plus opaque id refs;
  every reason from
  `batch_blocked_reason_class`);
- `unavailable_item_summary` (per-reason counts plus opaque id
  refs; every reason from `batch_unavailable_reason_class`);
- `count_truth` (per-term `collection_count_term` ↔
  `collection_count_status` plus value when applicable);
- `provider_side_query_scope` (when the execution origin is not
  `client_local_execution`):
  - `provider_filter_ast_id_ref`;
  - `provider_matching_count_value` plus `count_status`;
  - `provider_authoritative_basis_label`;
- `recovery_guidance_class`;
- `focus_target_id_ref` (opaque; the surface row / control focus
  returns to on close);
- `freshness_class`;
- `redaction_class`;
- `client_scopes`;
- `policy_context`;
- `running_build_identity_ref`;
- `minted_at`.

### Required population fields

The packet MUST carry, separately addressable:

1. **Current selection.** `included_item_id_refs` plus `selected`
   count and status.
2. **Matching set.** `matching` count and status; non-null when
   `selection_scope_class` is `all_matching_query` or
   `loaded_set`.
3. **Loaded subset.** `loaded` count and status.
4. **Provider-side query scope.** The
   `provider_side_query_scope` block; required when
   `batch_execution_origin_class` is
   `provider_authoritative_execution` or
   `mixed_client_then_provider`.

A packet missing any of the above for its execution origin denies
with `batch_provider_authoritative_count_missing_required` or
`batch_count_term_collapsed`.

### Identity stability (normative)

The included id refs are pinned at packet mint and MUST NOT change
between mint and apply. Filtering, sorting, virtualisation, or
provider re-ranking that would change the included set after mint
MUST invalidate the paired
`interaction_safety_packet_record` via
`interaction_safety_apply_basis_drifted` and reopen review. A
surface that silently widens the included set denies with
`batch_scope_widened_post_review`.

### Recovery guidance (normative)

Every packet names exactly one `batch_recovery_guidance_class`. A
packet whose paired interaction-safety consequence class is
`irreversible_high_blast` MAY name `no_recovery_available`; every
other consequence class MUST name a concrete recovery class.

## Cross-surface review rules

1. **No silent scope widening.** Filter changes, sort changes,
   virtualisation, and provider-cursor movement MUST NOT silently
   broaden the reviewed batch between mint and apply. Drift
   invalidates the paired packet and reopens review.
2. **Approximate stays approximate.** Provider-limited and
   approximate counts MUST remain so on copy, export, CLI output,
   support capture, deep link, and batch preview.
3. **Hidden selection stays inspectable.** `hidden_selected` and
   `not_loaded` counts MUST remain inspectable via keyboard at
   every step and MUST appear in the batch-review packet.
4. **Saved views degrade, not erase.** Drift MUST disclose,
   fallback MUST be named, and the user MUST decide whether to
   restore, recreate, or archive.
5. **Provider-owned authority stays attributed.** Provider-
   authoritative truth, provider-translated filters, and provider-
   only fields stay attributed even when the user also pinned a
   saved view, applied a local filter, or saved a workspace-shared
   bundle.
6. **Focus vs selection vs activation are distinct.** Focus is the
   cursor; selection is the admitted set; activation is the action
   trigger. A packet that conflates them denies with
   `batch_count_term_collapsed`.

## Surface guidance

| Surface family               | Default scope-truth class           | Default execution-origin class       | Required honesty cues                                                                                                            |
|------------------------------|-------------------------------------|--------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| `search_collection`          | `provider_authoritative_truth`      | `provider_authoritative_execution`   | provider matching count + status, hidden / blocked / unavailable summary, deep-link reopen-honesty pin                             |
| `review_collection`          | `mixed_truth_resolved_by_provider`  | `mixed_client_then_provider`         | apply-basis snapshot, blocked / partial / skipped breakout, focus-return target                                                    |
| `log_or_event_collection`    | `provider_authoritative_truth`      | `provider_authoritative_execution`   | retention-window disclosure, time range, freshness, unseen-tail count, recovery class on bulk export                              |
| `package_or_inventory_grid`  | `provider_authoritative_truth`      | `mixed_client_then_provider`         | publisher / version / ownership chips on every blocked row, recovery class on bulk uninstall                                      |
| `work_item_collection`       | `mixed_truth_resolved_by_provider`  | `provider_authoritative_execution`   | role-based visibility chips, ownership-blocked summary, cross-workspace identity stability                                         |
| `admin_or_settings_grid`     | `provider_authoritative_truth`      | `provider_authoritative_execution`   | policy-narrowing chip, evidence-only rows, change-window approval state, focus-return on commit                                    |

## Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `filter_ast_record`, `saved_view_record`, and
   `batch_review_packet_record` cross the RPC boundary as typed
   payloads. Raw bodies, raw paths, raw URLs, raw filter literal
   bytes, raw query bodies, and raw row text never cross.
2. Mutation-journal entries, save manifests, support bundles,
   evidence packets, and AI-context captures name `filter_ast_id`,
   `saved_view_id`, `batch_review_packet_id`,
   `paired_interaction_safety_packet_id_ref`, and
   `running_build_identity_ref` only.
3. CLI output and deep links re-resolve through the current
   filter-AST resolver and the current saved-view registry; the
   pinned filter AST and the pinned saved view are treated as
   *intent*, not as a frozen result claim.
4. Crash dumps and core files MUST NOT inherit unresolved batch-
   review packets; a crash that lands mid-apply discards the packet
   rather than persisting a partial axis set.

Redaction defaults (frozen):

| Sink                           | Default inclusion                                                                                                                                                       |
|--------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                   | Filter / saved-view / batch packet ids, surface class, scope-truth class, execution-origin class, count statuses, blocked / unavailable summary. No raw bodies or literals. |
| `traces_local`                 | Same as `logs_local`; span names MUST NOT include raw literals or row text.                                                                                              |
| `support_bundle`               | Full per-axis values, full count statuses, full blocked / unavailable enumeration, full filter compatibility classes, full saved-view drift state and fallback class.   |
| `evidence_packet`              | Release-relevant fields: `running_build_identity_ref`, scope-truth class, execution-origin class, recovery class, full id refs. Raw bodies / literals never included.     |
| `ai_context_capture`           | Filter / saved-view / batch packet ids, scope-truth class, execution-origin class, count statuses, recovery class. Raw bodies / literals never captured.                 |
| `recipe_manifest`              | `filter_ast_id`, `saved_view_id`, `batch_review_packet_id`, `running_build_identity_ref`. Raw bodies / literals forbidden.                                               |
| `profile_export` / `sync`      | Saved-view records gated by owner-class / scope-class / privacy-class. Local-only literals never replicated.                                                              |
| `mutation_journal_entry`       | Ids, scope-truth class, execution-origin class, recovery class. No raw bodies or literals.                                                                                |
| `claim_manifest`               | Full per-axis values, full filter compatibility classes, full saved-view drift state. Raw bodies / literals never included.                                              |
| `terminal_transcript`          | `batch_review_packet_id` and `surface_class` only.                                                                                                                       |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

## Schema-of-record posture

The eventual collection / shell crate's Rust types are the source
of truth. The JSON Schema exports at
`schemas/collections/filter_ast.schema.json`,
`schemas/collections/saved_view.schema.json`, and
`schemas/collections/batch_review_packet.schema.json` are the
cross-tool boundaries every non-owning surface reads.

Adding a new filter operator class, pattern dialect, source class,
field source class, literal handling class, compatibility class,
saved-view owner class, scope class, privacy class, fallback class,
drift state, batch execution origin, scope-truth class, item-
identity class, blocked reason, unavailable reason, recovery
class, count term, count status, denial reason, or audit-event id
is additive-minor and bumps
`filter_ast_schema_version`,
`saved_view_schema_version`, or
`batch_review_packet_schema_version` respectively.

Repurposing an existing member is breaking and requires a new
decision row in `artifacts/governance/decision_index.yaml`.

There is no external IDL or code-generator toolchain at this
milestone.

## Acceptance criteria cross-walk

This contract delivers the spec's five acceptance bullets:

1. **One collection contract across search, review, admin,
   package, log, and work-item surfaces.** The frozen
   `filter_ast_record`, `saved_view_record`, and
   `batch_review_packet_record` shapes plus the
   `collection_count_term`, `collection_count_status`,
   `selection_scope_class`, `batch_execution_origin_class`, and
   `batch_scope_truth_class` vocabularies cover every surface in
   scope.
2. **Saved views degrade visibly.** The `saved_view_drift_state`
   and `saved_view_fallback_class` vocabularies plus the
   `saved_view_drift_state_required_action_mismatch` and
   `saved_view_fallback_class_required_for_drift` denial reasons
   force every drifted view to disclose drift and pair to a
   fallback. Silent restore against drifted state denies.
3. **Batch-review fixtures cannot claim wider scope than the
   visible / loaded / matching / provider-side set.** The
   `count_truth` block, the
   `provider_side_query_scope` requirement on
   `provider_authoritative_execution` / `mixed_client_then_provider`,
   the
   `batch_provider_authoritative_count_missing_required` denial,
   and the
   `batch_scope_widened_post_review` denial cover the
   widening risk. The fixtures exercise visible-only,
   matching-set, provider-limited, blocked, and unavailable
   shapes.
4. **Saved views, exports, and batch actions cite one scope
   grammar and one blocked-count semantics.** The shared
   vocabularies above are owned by the three boundary schemas;
   no surface may mint a parallel value.
5. **Batch-review packets preserve stable item identity.** The
   `batch_item_identity_class` axis, the pinned
   `included_item_id_refs`, and the
   `batch_scope_widened_post_review` denial reason force the
   packet to invalidate and reopen review when sort, filter, or
   virtualisation would change the included set after mint.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- the live collection UI (filter builder, saved-view picker,
  batch-review sheet engine, column-pinning gestures, virtualised
  list / table rendering pipelines);
- the saved-view share UI, the org-share sync transport, and the
  managed-workspace provisioning pipeline;
- the CLI render layer for filter-AST display and CLI batch
  output;
- the eventual collection / shell / search crates' Rust types.

These lines move only by opening a new decision row, not by
editing this contract.

## Reuse guarantee

This contract is reusable by search, review, log, package, work-
item, and admin surfaces without redefining filter, saved-view,
or batch-review semantics. A new surface MUST:

1. emit a `filter_ast_record` for every persisted, exported,
   shared, or cross-surface filter and resolve every operator,
   dialect, and field source through the frozen vocabulary;
2. emit a `saved_view_record` for every saved view and resolve
   exactly one `saved_view_drift_state` and one
   `saved_view_fallback_class` on every restore that is not
   exact-bound;
3. emit a `batch_review_packet_record` before every
   destructive, provider-owned, remote, or export-bearing batch
   action, paired to exactly one
   `interaction_safety_packet_record`;
4. quote the `collection_count_term` and
   `collection_count_status` vocabularies verbatim on every
   visible count, copy, export, CLI line, deep link, and support
   capture;
5. pin `batch_item_identity_class` and the included id refs at
   mint and refuse silent widening between mint and apply;
6. honour the broker-owned redaction pass: opaque refs and
   reviewable labels only, no raw payloads.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — collection, filter, saved-view,
  batch-review, count-truth, and identity-stability requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  collection / filter / saved-view storage and resolution story.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — dense table /
  result grid / log viewer / batch-review UX rules.
- `docs/ux/shell_interaction_safety_contract.md` — interaction-
  safety packet and batch scope record this contract pairs to.
- `docs/ux/live_update_review_contract.md` — live-set state record
  this contract composes with.
- `docs/navigation/navigation_and_saved_query_contract.md` —
  saved-query bundle / search-collection-snapshot contract this
  contract composes with.
- `fixtures/ux/selection_and_virtualization_manifest.yaml` —
  selection-scope class and count-term vocabulary re-export.

## Linked artifacts

- Filter-AST boundary schema:
  `schemas/collections/filter_ast.schema.json`.
- Saved-view boundary schema:
  `schemas/collections/saved_view.schema.json`.
- Batch-review-packet boundary schema:
  `schemas/collections/batch_review_packet.schema.json`.
- Worked-example batch-review fixtures:
  `fixtures/collections/batch_review_examples/`.
- Interaction-safety contract:
  `docs/ux/shell_interaction_safety_contract.md`.
- Live-update review contract:
  `docs/ux/live_update_review_contract.md`.
- Navigation / saved-query contract:
  `docs/navigation/navigation_and_saved_query_contract.md`.
- Selection / virtualisation manifest:
  `fixtures/ux/selection_and_virtualization_manifest.yaml`.
