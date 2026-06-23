# Relation-navigation resolution — evidence companion

Human-readable companion to
[`/fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json`](../../fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json)
and its boundary schema
[`/schemas/navigation/relation_navigation_resolution.schema.json`](../../schemas/navigation/relation_navigation_resolution.schema.json).
It gives reviewers the frozen scenario and invariant tables without reading the
JSON. The contract narrative lives in
[`/docs/navigation/relation_navigation_resolution.md`](../../docs/navigation/relation_navigation_resolution.md).

- Set id: `relation-navigation-resolution:set:0001`
- Record kind: `relation_navigation_resolution_set`
- Scenarios: 7 · Invariants: 10

## Resolution scenarios

| Scenario | Command | Requested | Disposition | Aliasing posture | Navigated | Proves |
| --- | --- | --- | --- | --- | --- | --- |
| `definition_single_exact` | Go to Definition | definition | `resolved_single` | `no_alias` | definition | A single exact definition opens directly with no caveat. |
| `declaration_distinct_from_definition` | Go to Declaration | declaration | `resolved_single` | `no_alias` | declaration | A declaration resolves to the declaration only; a present definition is never picked. |
| `implementation_single` | Go to Implementation | implementation | `resolved_single` | `no_alias` | implementation | A sole implementation opens as a distinct implementation, not a definition. |
| `implementation_multi_disambiguation` | Go to Implementation | implementation | `opened_disambiguation` | `no_alias` | — | Three implementations open a disambiguation set instead of guessing. |
| `declaration_discloses_fallback` | Go to Declaration | declaration | `resolved_single` | `disclosed_fallback` | definition | With no declaration provider, the definition is a disclosed fallback; its kind stays `definition`. |
| `definition_lexical_fallback_disclosed` | Go to Definition | definition | `resolved_single` | `no_alias` | definition | A grep-only definition keeps its `lexical_fallback` proof and a downgrade reason. |
| `implementation_unavailable` | Go to Implementation | implementation | `unavailable` | `no_resolution` | — | No implementation found is reported unavailable, never aliased to a definition. |

The `disclosed_fallback` scenario carries a `missing_provider` downgrade reason
and a fallback note; its `navigated_relation` is `definition`, **not** the
requested `declaration`. The lexical scenario carries a `lexical_fallback_only`
downgrade reason. The unavailable scenario selects no target and opens no set.

## Dispositions and aliasing postures

| Disposition | Meaning |
| --- | --- |
| `resolved_single` | Exactly one admissible target was opened directly. |
| `opened_disambiguation` | More than one candidate; a disambiguation set was opened instead of guessing. |
| `unavailable` | No admissible target and no disclosed fallback. |

| Aliasing posture | Meaning |
| --- | --- |
| `no_alias` | The navigated relation kind equals the requested kind. |
| `disclosed_fallback` | A different relation kind was offered, disclosed with reasons; the kind is preserved, never relabeled. |
| `no_resolution` | The requested relation could not be served and no fallback was offered. |

There is deliberately no "silent alias" posture: the resolver can only preserve
the requested kind, disclose a different one, or decline.

## Frozen invariants (all `holds: true`)

- `relation_resolution.distinct_definition_declaration_implementation`
- `relation_resolution.never_relabels_relation_kind`
- `relation_resolution.multi_target_opens_disambiguation`
- `relation_resolution.no_silent_aliasing`
- `relation_resolution.disclosed_fallback_is_evidenced`
- `relation_resolution.fallback_proof_never_semantic`
- `relation_resolution.unavailable_is_honest`
- `relation_resolution.replayable_relation_and_reason`
- `relation_resolution.disambiguation_carries_truth`
- `relation_resolution.commands_covered_and_resolver_consistent`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/relation_navigation_resolution.rs`](../../crates/aureline-navigation/tests/relation_navigation_resolution.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals this fixture, re-proves that every stored resolution equals the resolver's
own output, that the corpus is support-export safe, that definition, declaration,
and implementation each resolve distinctly, and that every invariant holds — so a
claimed relation-kind navigation surface cannot promote while the resolver could
silently alias one relation kind for another.
