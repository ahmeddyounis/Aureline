# Relation-navigation resolution contract

How Aureline turns a **Go to Definition / Declaration / Implementation / Type
Definition** command into a distinct, relation-kind-explicit outcome — without
ever letting the commands silently alias one another.

- Resolver and corpus: [`crates/aureline-navigation/src/relation_resolution/mod.rs`](../../crates/aureline-navigation/src/relation_resolution/mod.rs)
- Boundary schema: [`schemas/navigation/relation_navigation_resolution.schema.json`](../../schemas/navigation/relation_navigation_resolution.schema.json)
- Frozen corpus: [`fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json`](../../fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json)
- Evidence companion: [`artifacts/navigation/relation_navigation_resolution.md`](../../artifacts/navigation/relation_navigation_resolution.md)
- Freeze gate: [`crates/aureline-navigation/tests/relation_navigation_resolution.rs`](../../crates/aureline-navigation/tests/relation_navigation_resolution.rs)

This contract sits on top of the typed
[navigation target model](m3/navigation_target_beta_contract.md) and the frozen
[relation-navigation matrix](m5-relation-navigation.md). Those name and qualify
the objects; this one governs the *resolution step* — choosing an outcome from
candidate targets.

## The three rules

A definition is not a declaration; an implementation is neither. The resolver
([`resolve_navigation`](../../crates/aureline-navigation/src/relation_resolution/mod.rs))
holds three rules so the commands stay trustworthy.

1. **Distinct relation kinds.** A command resolves only against candidates whose
   `relation_kind` equals the requested kind. Go to Declaration considers only
   declaration candidates even when a definition is present in the same result
   set, so the relation kinds never collapse into one another.
2. **Open disambiguation instead of guessing.** When more than one admissible
   candidate exists — where picking one over another could change behavior or
   meaning — the resolver opens a
   [disambiguation set](../../crates/aureline-navigation/src/relation_resolution/mod.rs)
   carrying provider, freshness, and ambiguity truth rather than selecting a best
   target silently.
3. **No silent aliasing.** When provider depth cannot serve the requested
   relation, the resolver either offers a **disclosed fallback** — the related
   target with its real relation kind preserved, a `missing_provider` downgrade
   reason, and a fallback note — or reports the command **unavailable**. It never
   relabels a different relation kind under the requested one.

## Resolution algorithm

Given a `NavigationCommand` and the candidate `NavigationTarget`s providers
returned for the origin symbol:

| Situation | Disposition | Aliasing posture | Outcome |
| --- | --- | --- | --- |
| Exactly one admissible candidate of the requested kind | `resolved_single` | `no_alias` | Opens the target; relation kind preserved. |
| Two or more admissible candidates of the requested kind | `opened_disambiguation` | `no_alias` | Opens a disambiguation set; no target is guessed. |
| No admissible candidate, requested kind has no provider, one conflatable related target | `resolved_single` | `disclosed_fallback` | Opens the related target with its real kind, `missing_provider`, and a fallback note. |
| No admissible candidate, requested kind has no provider, several conflatable related targets | `opened_disambiguation` | `disclosed_fallback` | Opens a disclosed-fallback disambiguation set. |
| No admissible candidate, requested kind *does* have a provider (it simply found nothing) | `unavailable` | `no_resolution` | Reports unavailable; never falls back to a different kind. |
| No admissible candidate and no conflatable related target | `unavailable` | `no_resolution` | Reports unavailable with `missing_provider`. |

A candidate is *admissible* when its `proof_class` is not `unavailable`. The
conflatable family is definition / declaration / implementation / type — the
kinds a fallback may be drawn from when provider depth is insufficient. Crucially,
when a provider *can* resolve the requested kind but returns nothing, the resolver
reports unavailable rather than substituting a sibling relation — the
distinction between "no such target" and "no such provider" is preserved in the
downgrade reasons.

## Objects

- **`NavigationResolution`** — the replayable outcome: the request id, the
  command, the requested relation, the disposition and aliasing posture, the
  navigated relation kind (present only when a target was opened), the selected
  target or disambiguation set, the considered candidate refs, aggregate
  provider/proof/freshness/ambiguity, an ambiguity count, downgrade reasons,
  fallback notes, and a one-sentence `replay_explanation`.
- **`RelationResolvedTarget`** — the distinct definition/declaration/
  implementation target: target ref and anchor, provider class, confidence,
  freshness, ambiguity class, an ambiguity count over its sibling candidates,
  authorship posture, downgrade reasons, and fallback notes — without rewriting
  the underlying target's relation kind.
- **`NavigationDisambiguationSet`** — reused from the target model: the candidate
  refs, a selection policy, and the provider/freshness/ambiguity truth a caller
  needs to choose.

`navigated_relation` equals `requested_relation` **only** under the `no_alias`
posture. Under `disclosed_fallback` it is the related target's real kind; under
`no_resolution` it is absent.

## Replay and support

Every resolution is metadata-only and serde-serializable, so search, graph,
docs/help, editor, AI, review, support, and CLI surfaces consume the same record.
Because each resolution echoes the request id and carries the disposition,
aliasing posture, navigated relation kind, and a `replay_explanation`, a support
or debug packet can reconstruct **which** relation kind Aureline navigated and
**why** — without any source body, raw path, provider payload, URL, hostname, or
credential. Refs are opaque `aureline://` handles or repo-relative paths only.

## Frozen invariants

The corpus computes each invariant's `holds` flag from the resolver's own output,
so an inconsistent change flips an invariant and fails CI:

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
equals the checked-in fixture, re-proves that every stored resolution equals the
resolver's own output, that the corpus is support-export safe, that definition,
declaration, and implementation each resolve distinctly, and that every frozen
invariant holds — so a claimed relation-kind navigation surface cannot promote
while the resolver could silently alias one relation kind for another.

Regenerate the fixture after any resolver change with:

```sh
cargo run -p aureline-navigation --example dump_relation_navigation_resolution \
  > fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json
```
