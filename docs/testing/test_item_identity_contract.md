# Test item identity, parameterized expansion, and selector grammar contract

This contract freezes the identity layer beneath Aureline test
discovery, test execution, watch mode, inline markers, AI plans,
imported provider overlays, support exports, and release evidence.
Its job is simple: every surface that points at a runnable test points
at the same durable object, or at a typed remap record that explains
why exact identity could not be preserved.

Display names are not identity. A test row may be renamed, localized,
trimmed for a tree, rendered with a parameter label, or imported from a
provider with different casing. Those display projections are useful to
humans, but they are never the key used for rerun, debug, quarantine,
watch history, evidence packets, or AI tool plans.

Machine-readable companions:

- [`/schemas/testing/test_item_identity.schema.json`](../../schemas/testing/test_item_identity.schema.json)
  - the `test_item_identity_record`,
  `parameterized_expansion_record`, and
  `test_item_identity_remap_record`.
- [`/schemas/testing/test_selector_grammar.schema.json`](../../schemas/testing/test_selector_grammar.schema.json)
  - the parsed selector AST, escaping profile, target binding,
  resolution, matched canonical identities, remap refs, omitted
  scopes, and denial reasons.
- [`/fixtures/testing/test_item_identity_cases/`](../../fixtures/testing/test_item_identity_cases/)
  - worked YAML fixtures covering native adapter identity, imported CI
  remap, parameterized family expansion, tag selectors, and renamed
  test files.

This contract composes with and does not replace:

- [`/docs/execution/test_truth_contract.md`](../execution/test_truth_contract.md),
  [`/schemas/execution/test_discovery_state.schema.json`](../../schemas/execution/test_discovery_state.schema.json),
  and
  [`/schemas/execution/test_run_summary.schema.json`](../../schemas/execution/test_run_summary.schema.json).
  The testing identity schema owns durable identity and selection. The
  execution schemas own discovery state, per-item state, run summaries,
  coverage handoff, snapshot review, flake history, and quarantine.
- [`/docs/execution/test_watch_and_environment_contract.md`](../execution/test_watch_and_environment_contract.md),
  [`/schemas/execution/watch_controller_state.schema.json`](../../schemas/execution/watch_controller_state.schema.json),
  [`/schemas/execution/inline_test_result.schema.json`](../../schemas/execution/inline_test_result.schema.json),
  and
  [`/schemas/execution/environment_matrix_row.schema.json`](../../schemas/execution/environment_matrix_row.schema.json).
  Watch, inline marker, and environment rows cite identity or remap
  refs; they do not mint local test IDs.
- [`/docs/testing/test_session_and_attempt_contract.md`](./test_session_and_attempt_contract.md),
  [`/schemas/testing/test_session.schema.json`](../../schemas/testing/test_session.schema.json),
  and
  [`/schemas/testing/test_attempt.schema.json`](../../schemas/testing/test_attempt.schema.json).
  Sessions and attempts preserve selector, target/environment,
  source, artifact, raw-event, time, rerun, debug-from-test, and
  imported-provider lineage around those identities. They cite
  canonical identities and selector records rather than matching
  display labels.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md),
  [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json),
  and
  [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json).
  Test identity includes target and environment identity because a
  runnable unit on local, container, remote, managed, notebook, or CI
  targets may not be comparable without re-resolution.
- [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md),
  [`/docs/navigation/semantic_navigation_and_rename_contract.md`](../navigation/semantic_navigation_and_rename_contract.md),
  and [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md).
  Source moves and generated-output regeneration use those source and
  lineage truths before this contract records test remap state.

Raw command lines, raw stdout or stderr byte streams, raw environment
bodies, raw absolute paths, raw URLs, raw secret values, raw assertion
bodies, raw source excerpts, raw artifact bytes, and raw stack traces
MUST NOT cross these boundaries. Records carry opaque refs, digests,
counts, bounded display summaries, timestamps, and class labels.

## Canonical Identity

Every runnable or reviewable test item has one
`canonical_test_item_id`. The id is opaque and safe to copy into
editor state, test-tree rows, CLI structured output, watch histories,
AI tool plans, imported CI overlays, support bundles, and release
evidence.

The canonical identity tuple is the combination of:

| Field group | Required meaning |
|---|---|
| Logical path | Stable suite/container path plus stable case path. Each path segment carries a `stable_segment_key` and a source class such as framework stable id, source symbol anchor, provider case id, notebook cell id, or generated lineage id. |
| Parameterization | `not_parameterized`, family root, concrete instance, runtime-enumerated instance, generated instance, or provider-imported instance, with a family key and instance key where applicable. |
| Source location span | Source provider class, source ref, optional source revision ref, optional line/column span, anchor digest, and span fidelity. A raw path or display label is not a source identity. |
| Adapter/provider | Adapter kind, adapter ref, adapter version ref where known, framework family, provider authority, and optional raw-payload ref on the artifact rail. |
| Target environment | Target class, execution context ref, context snapshot ref, environment fingerprint, runtime/toolchain/build refs where known, and target support class. |
| Display projection | Human label, label digest, label source, sort-key digest, locale ref, and the constant role `display_only_not_identity`. |

The identity row may project to an execution
`test_item_record`, but the projection is not the identity. Execution
rows can be refreshed, filtered, stale, quarantined, or imported while
the canonical identity persists.

## Parameterized Cases

Parameterized tests use two related identities:

- A family identity for the template/root. It is selectable even when
  exact instance enumeration is deferred.
- One identity per concrete invocation when the adapter, provider, or
  runtime can name an instance.

Concrete instances MUST cite their parameterized family. Their
`parameterization_key` MUST name how the instance is addressed:
stable named case key, normalized arguments digest, matrix axis tuple,
provider case id, runtime generated key, or ordinal index only. Ordinal
index alone is review-only after reorder unless paired with a stronger
source or provider anchor.

The `parameterized_expansion_record` is what lets tree views stay
usable without losing precision:

- `expansion_policy_class` says whether instances are eagerly known,
  lazy-loaded, loaded only on failure, runtime-only, imported-only, or
  shown through a bounded preview.
- `collapse_policy_class` says whether a family is collapsed by
  default, expanded on failure, expanded by user request, expanded by
  default for small families, or never collapsed.
- `parameterized_counts` preserve known, loaded, passed, failed,
  errored, skipped, quarantined, and unknown instance counts even when
  the UI collapses children.
- `failing_instance_refs` keep exact failing cases reachable from
  collapsed rows.
- `rerun_binding_class` says whether rerun/debug can bind exact
  instance refs, must bind the family with expansion policy, requires
  review before widening, or is denied.

`Rerun failed` and watch-mode retry history MUST cite exact instance
refs when those refs exist. If an adapter can only rerun a family, the
action is a widened selection and must show the review reason before
execution. Evidence packets may cite exact instances, the grouped
family, or a mixed family/instance view, but they must say which.

## Selector Grammar

Selectors are normalized into an AST before they cross a contract
boundary. User-facing CLI text, tree filters, saved views, and AI plans
may use friendly syntax, but exported records carry
`test_selector_expression_record` with a selector digest, AST, escape
profile, target binding, import-safety class, and resolution result.

Normative surface grammar:

```text
selector      := expr
expr          := atom | all "(" expr-list ")" | any "(" expr-list ")"
               | not "(" expr ")" | difference "(" expr "," expr ")"
expr-list     := expr ("," expr)*
atom          := id ":" value
               | file ":" value
               | suite ":" value
               | case ":" value
               | tag ":" value
               | param ":" family-key "[" instance-key "]"
               | param-family ":" family-key
               | trait ":" value
               | source ":" source-provider-class
               | adapter ":" adapter-kind
               | target ":" target-class
               | changed-since ":" snapshot-ref
value         := escaped-token
```

Supported selector classes:

| Selector | Meaning |
|---|---|
| `id:` | Exact canonical test-item id. |
| `file:` | Workspace-relative source ref, generated source ref, notebook ref, or provider artifact ref. Raw absolute paths are not admitted into machine-readable selector records. |
| `suite:` | Logical suite/container path prefix. |
| `case:` | Logical case path or exact case key. |
| `tag:` | Tag membership from adapter, framework, provider import, or saved test metadata. |
| `param:` | Exact parameterized instance under a family key. |
| `param-family:` | Parameterized family root with expansion/collapse policy preserved. |
| `trait:` | Capability or trait such as run, debug, coverage, watch, rerun-failed, quarantine, snapshot review, source jump, evidence export, or imported read-only. |
| `source:` | Source-provider class such as workspace VFS file, generated lineage, notebook cell source, structured test report import, provider CI result, build event stream, or symbol-index projection. |
| `adapter:` | Adapter kind. |
| `target:` | Target environment class. |
| `changed-since:` | Snapshot-scoped query for impacted or changed files/tests. |

Selector atoms match durable fields, not display labels. A selector
that can only match a display label resolves to
`partial_requires_review` or `unsupported_selector_denied`; it may not
silently broaden to a suite or file.

### Escaping Rules

Import-safe selectors use the escape profile stored on the record.
The default profile is:

- normalize imported text to NFC before escaping, unless a provider
  payload must preserve provider normalization and carry a digest;
- backslash-escape reserved ASCII characters:
  `\`, `:`, `/`, `[`, `]`, `(`, `)`, `,`, `*`, `?`, and whitespace
  when it would be ambiguous;
- encode control characters as `\u{hex}`;
- percent-decode provider values before applying the Aureline escape
  profile only when the import row says the provider percent-encoded
  those values;
- store raw absolute paths and raw URLs as refs or digests, not as
  selector values.

Escaping is import safety, not identity. The selector record always
names the atom class, match semantics, and durable value refs or
digests so future parsers can explain what matched.

## Remap And Drift

When a known test cannot be resolved at the old source or provider
location, the resolver emits `test_item_identity_remap_record`.

Remap inputs may include:

- framework stable ids that survived a file move or suite rename;
- source anchors and context digests from the VFS or navigation layer;
- generated lineage rows after regenerated tests;
- provider case ids and structured test reports from imported CI;
- target/environment fingerprints from local, container, remote,
  managed, notebook, or CI rows;
- selector records that were exact before the drift.

Remap confidence classes:

| Class | Behavior |
|---|---|
| `exact_framework_stable_id` | Preserve the canonical id and append remap evidence. |
| `exact_source_anchor` | Preserve the canonical id or keep an alias to the successor when the source moved. |
| `exact_provider_case_id` | Preserve imported evidence as read-only until local parity maps it to a workspace source. |
| `probable_context_digest` | Keep the prior row visible, require review before rerun/debug, and cite evidence. |
| `ambiguous_requires_review` | Do not choose a target automatically. Preserve the prior row and show candidates through remap review. |
| `missing_tombstone` | Preserve a tombstone with no rerun/debug action until a successor is found. |
| `unsupported_selector_denied` | Deny the selector without widening scope. |

Drift behavior is explicit:

- `keep_canonical_id` for exact remaps;
- `keep_alias_to_successor` when a new identity is minted but old refs
  still need to resolve;
- `require_review_before_rerun` for probable or ambiguous matches;
- `preserve_read_only_evidence` for imported provider rows that cannot
  yet run locally;
- `tombstone_no_rerun` for deleted or missing tests;
- `deny_without_widening` for unsupported selectors.

Imported provider results remain overlays until a local or target-bound
identity row can be matched. A provider pass/fail row may decorate the
tree or editor as imported/stale/read-only evidence, but it may not
become a live local result without a target-qualified run summary.

## Required Invariants

| Invariant | Contract surface |
|---|---|
| One runnable unit has one canonical identity across editor, tree, CLI, watch, AI, imported CI, support, and release evidence. | `test_item_identity_record.canonical_test_item_id` plus remap records. |
| Human display is separated from identity. | `display_projection.display_identity_role = display_only_not_identity`. |
| Parameterized families and concrete instances remain distinct. | `parameterization_kind_class`, `parameterized_family_ref`, and `parameterized_expansion_record`. |
| Collapsed parameterized rows preserve hidden counts and failing-instance access. | `parameterized_counts` and `failing_instance_refs`. |
| Selectors never match display labels alone. | `test_selector_expression_record` denial vocabulary. |
| File selectors do not serialize raw absolute paths or URLs. | `escape_profile.path_policy_class` and file selector match values. |
| Rerun/debug does not silently widen after drift. | `remap_confidence_class`, `drift_behavior_class`, and selector resolution. |
| Imported provider rows remain read-only or parity-qualified. | `provider_authority_class`, `target_support_class`, and remap records. |
| Missing tests preserve tombstones until a successor or reviewed deletion exists. | `missing_tombstone` plus `tombstone_no_rerun`. |

## Versioning

The two schema files declare `*_schema_version = 1`.

Adding a new optional field, enum member, selector atom, or record kind
is additive-minor and bumps the relevant schema version. Removing or
repurposing an existing value is breaking and requires a decision row.
Any schema change that changes identity semantics must update this
document and the fixture corpus in the same change.
