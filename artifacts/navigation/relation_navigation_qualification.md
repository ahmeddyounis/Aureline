# Relation-navigation qualification — evidence companion

Human-readable companion to
[`/fixtures/navigation/relation_navigation_qualification/canonical_certification.json`](../../fixtures/navigation/relation_navigation_qualification/canonical_certification.json)
and its boundary schema
[`/schemas/navigation/relation_navigation_qualification.schema.json`](../../schemas/navigation/relation_navigation_qualification.schema.json).
It gives reviewers the frozen family, row, release-evidence, and invariant tables
without reading the JSON. The contract narrative lives in
[`/docs/navigation/relation_navigation_qualification.md`](../../docs/navigation/relation_navigation_qualification.md).

- Certification id: `relation-navigation-qualification:certification:0001`
- Record kind: `relation_navigation_qualification_certification`
- Families: 6 · Qualification rows: 19 · Release-evidence rows: 6 · Consumers: 7 · Invariants: 10

## Certified relation-navigation families

Each family certifies one or more relation-navigation matrix objects, names the
producing module, binds a boundary schema, and maps to the proof packet and freeze
gate that keep it current.

| Family | Certified matrix objects | Proof packet | Freeze gate |
| --- | --- | --- | --- |
| `target_kind_honesty` | navigation_target, relation_fallback_vocabulary | `fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json` | `crates/aureline-navigation/tests/relation_navigation_resolution.rs` |
| `reference_access_kind_truth` | reference_occurrence | `fixtures/navigation/reference_panes/canonical_panes.json` | `crates/aureline-navigation/tests/reference_panes.rs` |
| `hierarchy_proof_classes` | hierarchy_edge | `fixtures/navigation/hierarchy_views/canonical_views.json` | `crates/aureline-navigation/tests/hierarchy_views.rs` |
| `related_object_attribution` | related_object_relation | `fixtures/navigation/related_object_navigation/canonical_links.json` | `crates/aureline-navigation/tests/related_object_navigation.rs` |
| `rename_preview_completeness` | rename_preview_set | `fixtures/navigation/governed_rename_preview/canonical_previews.json` | `crates/aureline-navigation/tests/rename_preview.rs` |
| `continuity_replay_fidelity` | navigation_target, reference_occurrence | `fixtures/navigation/relation_continuity/canonical_continuity.json` | `crates/aureline-navigation/tests/relation_continuity.rs` |

Every family is bound to the relation-navigation matrix
([`m5-relation-navigation`](./m5-relation-navigation.md)) by its
`relation_nav_object.*` object ids.

## Claimed surfaces and row coverage

Each family backs the surfaces below; the certification publishes one qualification
row per `(family, surface)` pair (19 rows total).

| Family | search / navigation | graph / topology | docs / help | editor assist |
| --- | --- | --- | --- | --- |
| `target_kind_honesty` | ✓ | ✓ | ✓ | ✓ |
| `reference_access_kind_truth` | ✓ | ✓ | | ✓ |
| `hierarchy_proof_classes` | | ✓ | ✓ | ✓ |
| `related_object_attribution` | ✓ | ✓ | ✓ | ✓ |
| `rename_preview_completeness` | ✓ | | | ✓ |
| `continuity_replay_fidelity` | ✓ | | ✓ | ✓ |

## Claim-state vocabulary and auto-narrowing

Every row's `claim_state` is derived from its `proof_state` and `proof_freshness`
by the narrowing function — it is never authored directly. In the canonical
certification every family is `passing` + `live`, so every claim is `qualified`.

| `proof_state` | `proof_freshness` | derived `claim_state` |
| --- | --- | --- |
| `passing` | `live` / `warm` | `qualified` |
| `passing` | `degraded` | `narrowed_disclosed` |
| `passing` | `stale` | `narrowed_stale` |
| `passing` | `unverified` | `withdrawn_pending_proof` |
| `pending` / `missing` | any | `withdrawn_pending_proof` |
| `failing` | any | `withdrawn_failing` |

Only `qualified` renders without a caveat; a `narrowed_*` row carries a disclosure
note, and a `withdrawn_*` row withdraws the claim. A surface aggregates to the
most-severe claim across the families backing it, so a single stale or failing
family narrows the affected search/graph/docs/editor claim automatically.

## Release evidence rows

The release-evidence packet states one row per family. The five families a release
packet must name explicitly are target-kind honesty, references/access-kind truth,
hierarchy proof classes, related-object attribution, and rename-preview
completeness; continuity/replay fidelity is also included.

| Evidence id | Guarantee | Holds (canonical) |
| --- | --- | --- |
| `relation_nav_qual_evidence.target_kind_honesty` | A definition jump is never relabeled a declaration and a grep fallback is disclosed. | yes |
| `relation_nav_qual_evidence.reference_access_kind_truth` | Read/write/call/test-only/generated occurrences keep their access kind and proof class. | yes |
| `relation_nav_qual_evidence.hierarchy_proof_classes` | Direct/transitive/inferred/runtime-observed edges preserve proof class and ambiguity. | yes |
| `relation_nav_qual_evidence.related_object_attribution` | Every related-object link is source-attributed and disambiguable. | yes |
| `relation_nav_qual_evidence.rename_preview_completeness` | Blocked/generated/read-only/partial-scope candidates are exposed before any broad mutation. | yes |
| `relation_nav_qual_evidence.continuity_replay_fidelity` | Peek/reveal/split/history entries preserve relation kind and target truth across replay and drift. | yes |

## Consumer surfaces

`about`, `help`, `search_navigation`, `support`, `compatibility`, `release_truth`,
`public_truth`. Every consumer projection sets `consumes_shared_state: true` and
`restates_manually: false`.

## Frozen invariants (all `holds: true`)

- `relation_nav_qual.every_family_certified`
- `relation_nav_qual.family_binds_matrix_object`
- `relation_nav_qual.every_family_maps_proof`
- `relation_nav_qual.narrowing_applied`
- `relation_nav_qual.no_green_claim_without_current_proof`
- `relation_nav_qual.narrowed_rows_disclose`
- `relation_nav_qual.every_surface_governed`
- `relation_nav_qual.release_evidence_covers_named_families`
- `relation_nav_qual.consumers_share_state`
- `relation_nav_qual.stable_ids_unique`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/relation_navigation_qualification.rs`](../../crates/aureline-navigation/tests/relation_navigation_qualification.rs)
runs under `cargo test --workspace`. It rebuilds the certification in code and
asserts it equals this fixture byte-for-byte, re-proves support-export safety and
full family/surface/consumer coverage, asserts every family maps to a proof packet
and freeze gate, and proves the narrowing function is applied to every row and that
no row stays green without current, passing proof — so a claimed search/graph/docs/
editor relation-navigation surface cannot promote without current proof, and a stale
or failing family narrows or withdraws the affected claim automatically.
