# Relation-navigation qualification contract

This document freezes the claim-governance lane that binds Aureline's relation-kind
navigation and rename-preview truth into M5 promotion. It certifies each claimed
search/graph/docs/editor relation-navigation family and **auto-narrows the claim
when the proof behind it goes stale or fails** — so a green claim can never outlive
its proof.

The certification does not re-implement relation-navigation truth. The
[relation-navigation matrix](./m5-relation-navigation.md) freezes the object model,
and each sibling lane —
[`relation_resolution`](../../crates/aureline-navigation/src/relation_resolution/mod.rs),
[`reference_panes`](../../crates/aureline-navigation/src/reference_panes/mod.rs),
[`hierarchy_views`](../../crates/aureline-navigation/src/hierarchy_views/mod.rs),
[`related_object_navigation`](../../crates/aureline-navigation/src/related_object_navigation/mod.rs),
[`rename_preview`](../../crates/aureline-navigation/src/rename_preview/mod.rs), and
[`relation_continuity`](../../crates/aureline-navigation/src/relation_continuity/mod.rs)
— produces the typed records. This lane is the single place that **names the
certified relation-navigation families**, **binds each to the matrix object and the
proof packet plus freeze gate that keep it current**, **publishes a qualification
row per claimed surface whose claim state is derived purely from its proof state and
freshness**, **emits explicit release evidence rows**, and **projects to the
consumer surfaces** that consume the same state — so search, graph, docs/help, and
editor claims degrade automatically when proof is stale or failing rather than
staying green because query sessions, result ids, or hover/peek basics still exist.

The track invariant this lane protects: **relation kinds stay explicit and
trustworthy, and a claim never outlives its proof.** A definition is not a
declaration; a grep fallback never masquerades as semantic certainty; implementation
and hierarchy edges preserve their proof class and ambiguity; related-object
navigation stays source-attributed; rename preview exposes blocked, generated,
read-only, and partial-scope candidates before any broad mutation; and continuity
entries preserve relation kind and target truth across replay and drift.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update in
the same change.

## Companion artifacts

- [`/schemas/navigation/relation_navigation_qualification.schema.json`](../../schemas/navigation/relation_navigation_qualification.schema.json)
  — boundary schema for `relation_navigation_qualification_certification`.
- [`/fixtures/navigation/relation_navigation_qualification/canonical_certification.json`](../../fixtures/navigation/relation_navigation_qualification/canonical_certification.json)
  — the published canonical certification; the freeze gate asserts the in-code
  builder equals it byte-for-byte.
- [`/artifacts/navigation/relation_navigation_qualification.md`](../../artifacts/navigation/relation_navigation_qualification.md)
  — the human-readable companion (family, row, evidence, and invariant tables).
- `crates/aureline-navigation/src/relation_navigation_qualification/` — the
  builder, the narrowing function, invariants, validation, and projection.
- `cargo run -p aureline-navigation --example dump_relation_navigation_qualification`
  — the headless emitter (JSON, or `-- --lines` for the projection).

## Certified families

Each family certifies one or more relation-navigation matrix objects, is produced by
an existing lane module, binds a boundary schema, and maps to a proof packet and the
freeze gate that re-checks it under `cargo test --workspace`.

| Family token | Certifies (matrix objects) | Proof packet |
| --- | --- | --- |
| `target_kind_honesty` | navigation_target, relation_fallback_vocabulary | `fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json` |
| `reference_access_kind_truth` | reference_occurrence | `fixtures/navigation/reference_panes/canonical_panes.json` |
| `hierarchy_proof_classes` | hierarchy_edge | `fixtures/navigation/hierarchy_views/canonical_views.json` |
| `related_object_attribution` | related_object_relation | `fixtures/navigation/related_object_navigation/canonical_links.json` |
| `rename_preview_completeness` | rename_preview_set | `fixtures/navigation/governed_rename_preview/canonical_previews.json` |
| `continuity_replay_fidelity` | navigation_target, reference_occurrence | `fixtures/navigation/relation_continuity/canonical_continuity.json` |

The `relation_nav_qual.family_binds_matrix_object` invariant fails if any family
cites no matrix object, producer, or schema, so the certification can never assert a
guarantee that is not anchored in the object model.

## Claimed surfaces and qualification rows

The certification publishes one qualification row per `(family, claimed surface)`
pair across the four claimed M5 profiles — `search_navigation`, `graph_topology`,
`docs_help`, and `editor_assist` (19 rows). Each row carries its proof packet, proof
state, proof freshness, the computed claim state, and — when narrowed or withdrawn —
a narrowing reason and disclosure note. The `relation_nav_qual.every_surface_governed`
invariant fails if any claimed surface has no row.

## Auto-narrowing: a claim never outlives its proof

A row's `claim_state` is **derived**, never authored, by the narrowing function in
[`relation_navigation_qualification`](../../crates/aureline-navigation/src/relation_navigation_qualification/mod.rs):

| `proof_state` | `proof_freshness` | derived `claim_state` |
| --- | --- | --- |
| `passing` | `live` / `warm` | `qualified` |
| `passing` | `degraded` | `narrowed_disclosed` |
| `passing` | `stale` | `narrowed_stale` |
| `passing` | `unverified` | `withdrawn_pending_proof` |
| `pending` / `missing` | any | `withdrawn_pending_proof` |
| `failing` | any | `withdrawn_failing` |

Two invariants pin this:

- `relation_nav_qual.narrowing_applied` — every row's claim state equals the
  narrowing of its proof state and freshness, so a claim can never be authored green
  independently of its proof.
- `relation_nav_qual.no_green_claim_without_current_proof` — no row is `qualified`
  unless its proof is `passing` and its freshness is current, so a stale, unverified,
  pending, missing, or failing proof never leaves a claim green.

A surface aggregates to the **most-severe** claim across the families backing it, so
one stale or failing family narrows the affected search/graph/docs/editor claim
automatically. `relation_nav_qual.narrowed_rows_disclose` guarantees every narrowed
or withdrawn row carries a reason and a disclosure note, and every qualified row
carries neither.

## Release evidence

The certification emits one release-evidence row per family stating the relation-kind
honesty guarantee and whether it currently holds.
`relation_nav_qual.release_evidence_covers_named_families` fails unless an explicit,
proof-consistent row exists for definition/declaration/implementation honesty,
references/access-kind truth, hierarchy proof classes, related-object attribution,
and rename-preview completeness — so a release packet always states each guarantee
and whether its proof is current.

## Consumer surfaces

The lane lists the surfaces that consume the qualification state instead of restating
relation-navigation quality claims by hand: `about`, `help`, `search_navigation`,
`support`, `compatibility`, `release_truth`, and `public_truth`. The
`relation_nav_qual.consumers_share_state` invariant fails if any consumer does not
consume the shared state or restates claims manually, so About/help/search/support/
compatibility/release/public-truth surfaces all read the same certification.

## Invariants and release-automation binding

[`relation_navigation_qualification`](../../crates/aureline-navigation/src/relation_navigation_qualification/mod.rs)
computes each invariant's `holds` flag from the built families, rows, evidence, and
projections, so the checked-in fixture and the freeze gate freeze the contract
byte-for-byte and an inconsistent edit flips an invariant and fails CI.

The release-automation binding is the freeze gate
[`crates/aureline-navigation/tests/relation_navigation_qualification.rs`](../../crates/aureline-navigation/tests/relation_navigation_qualification.rs),
which runs under `cargo test --workspace`. The invariant
`relation_nav_qual.every_family_maps_proof` flips false the moment a claimed family
lacks a mapped proof packet or freeze gate, so stable promotion cannot harden a
relation-navigation claim without current proof.

The frozen invariants:

- `relation_nav_qual.every_family_certified` — every family present exactly once.
- `relation_nav_qual.family_binds_matrix_object` — every family cites a matrix
  object, a producer, and a schema.
- `relation_nav_qual.every_family_maps_proof` — every family maps to a proof packet
  and freeze gate.
- `relation_nav_qual.narrowing_applied` — every row's claim state equals the
  narrowing of its proof.
- `relation_nav_qual.no_green_claim_without_current_proof` — no row is green without
  current passing proof.
- `relation_nav_qual.narrowed_rows_disclose` — narrowed and withdrawn rows disclose;
  qualified rows do not.
- `relation_nav_qual.every_surface_governed` — every claimed surface has a row.
- `relation_nav_qual.release_evidence_covers_named_families` — release evidence names
  the five required families consistently.
- `relation_nav_qual.consumers_share_state` — every consumer consumes the shared
  state without restating manually.
- `relation_nav_qual.stable_ids_unique` — family, row, and evidence ids are unique.

## Export safety

The record carries no source bodies, raw paths, provider payloads, URLs, hostnames,
or credentials — only opaque object refs, stable tokens, and short reviewable
sentences. `raw_payload_excluded` is always `true` and every ref is a repo-relative
object ref, so the certification is safe to embed in a support export verbatim.
