# Governed rename-preview contract

How Aureline turns a **broad rename** into an inspectable change object that shows
*what the rename would change*, *what it would not change and why*, and an
**inspect-before-mutate apply gate** — instead of one opaque apply button that
silently drops the candidates it cannot or will not touch.

- Builder and corpus: [`crates/aureline-navigation/src/rename_preview/mod.rs`](../../crates/aureline-navigation/src/rename_preview/mod.rs)
- Boundary schema: [`schemas/navigation/governed_rename_preview.schema.json`](../../schemas/navigation/governed_rename_preview.schema.json)
- Frozen corpus: [`fixtures/navigation/governed_rename_preview/canonical_previews.json`](../../fixtures/navigation/governed_rename_preview/canonical_previews.json)
- Evidence companion: [`artifacts/navigation/governed_rename_preview.md`](../../artifacts/navigation/governed_rename_preview.md)
- Freeze gate: [`crates/aureline-navigation/tests/rename_preview.rs`](../../crates/aureline-navigation/tests/rename_preview.rs)

This contract sits on top of the typed
[navigation target model](m3/navigation_target_beta_contract.md) — which freezes the
[`RenamePreviewSet`](../../crates/aureline-navigation/src/target_model/mod.rs) object
and the `RenameApplyPosture` — and the frozen
[relation-navigation matrix](m5-relation-navigation.md), which names the
rename-preview object family and pins its rename-omission vocabulary. Those qualify
the candidate set; this contract governs the **preview-and-apply model** that turns it
into a trustworthy, reviewable broad rename.

> This is a separate contract from the language-wedge
> [`rename_preview_record`](semantic_navigation_and_rename_contract.md) packet. That
> packet is a single rename's provider-level record; this corpus is the governed
> preview-and-apply model over the rename-preview-set object.

## What a governed rename preview shows

The builder
([`build_rename_preview`](../../crates/aureline-navigation/src/rename_preview/mod.rs))
is a pure function over a `RenamePreviewInput`. Given the rename candidates for a root
symbol it produces a `GovernedRenamePreview` with five guarantees.

1. **Disjoint candidate grouping.** Candidates are split into `RenameCandidateGroup`s
   keyed by `RenameCandidateGroupKind` by a fixed precedence: **blocked → conflict →
   generated → read-only/external → partial-scope → editable** (the editable group is
   listed first). The first matching rule wins, so a blocked, conflicting, generated,
   read-only, or out-of-scope candidate is never folded into the editable set, and
   every candidate lands in exactly one group. Only the **editable** group is mutated
   by an apply.
2. **Change-versus-held counts.** Each group and the preview carry a
   `RenameCandidateCounts` separating the editable set the rename *will change*
   (`will_change_count`) from the held set it *will not* (blocked/conflict/generated/
   read-only/partial-scope), plus `current_scope_count` vs `captured_scope_count` and
   an `unresolved_count`. The disjoint group counts always sum to `total_count`, and
   `current + captured == total`.
3. **Omission and conflict truth.** Every non-editable candidate keeps a visible
   `RenameOmissionReason` on its group and a visible `RenameCandidateLabel`; conflict
   candidates keep their conflict notes; and any group resting on a lexical/grep or
   syntax fallback carries a fallback note **and** a downgrade reason, so an omitted
   candidate never disappears and a grep match is never renamed as if semantic.
4. **Inspect-before-mutate apply gate.** Each preview carries a `RenameApplyGate` with
   `inspect_before_mutate_required: true` and `blind_apply_blocked: true` **always**.
   It derives a `RenameApplyPosture`, the `RenameApplyPrecondition`s to clear, keeps
   omitted and redacted candidates visible, and binds an `undo_checkpoint_ref`. Apply
   is allowed only when the posture is `ready_for_apply_after_preview`.
5. **Consumer parity.** Each preview projects to every `ConsumerSurface` with a
   `RenamePreviewProjection` that preserves the grouping, counts, omission reasons,
   conflict notes, apply gate, and undo checkpoint, keeps omitted candidates visible,
   never flattens the rename into one apply action, and never exports raw code bodies.

## Group precedence and the apply posture

| Group | Token | Mutated on apply? | Why it is held |
| --- | --- | --- | --- |
| Editable | `editable` | yes | Will be renamed after preview. |
| Blocked | `blocked_for_review` | no | Policy/protected/blocked — pending review. |
| Conflict | `conflict` | no | Shadowing or alias collision — pending resolution. |
| Generated boundary | `generated_boundary` | no | Generated/paired artifact — rename the source. |
| Read-only / external | `read_only_or_external` | no | Read-only, external, or imported — cannot mutate. |
| Partial scope | `partial_scope_omitted` | no | Out of scope, sparse, stale, or unresolved anchor. |

The apply posture is derived from the group state, so it can never claim a rename is
safe to apply while candidates are held:

| Posture | When |
| --- | --- |
| `ready_for_apply_after_preview` | An editable set exists with no blocking group and no refresh need. |
| `blocked_pending_policy_or_protected_review` | A blocked candidate carries a policy/protected reason. |
| `blocked_pending_scope_review` | A conflict (or non-policy block) is present. |
| `blocked_pending_refresh` | A candidate's scope is degraded, stale, or unverified. |
| `inspect_only_unavailable` | Nothing is editable — every candidate is held. |

## Inspect before mutate

`inspect_before_mutate_required` and `blind_apply_blocked` are **always true**, so a
broad rename can never apply without the preview being inspected. `RenameApplyPrecondition`s
name what must be cleared first — `review_blocked_candidates`, `resolve_conflicts`,
`acknowledge_generated_boundary`, `acknowledge_read_only_omission`,
`acknowledge_partial_scope`, `refresh_stale_scope`, `widen_sparse_scope`. The gate
binds an `undo_checkpoint_ref` so the preview is replayable and the apply is
reversible, and it states that `omitted_candidates_remain_visible` and
`redacted_candidates_remain_visible` even when content is redacted or out of scope.

## Evidence honesty

Each group and the preview name a `RenameEvidenceClass` — `semantic`,
`framework_derived`, `runtime_observed`, `imported_snapshot`, `lexical_fallback`,
`syntax_fallback`, `mixed`, or `unavailable`. A candidate proven only by an imported
snapshot or a runtime trace is captured-scope only and labeled as such; a candidate
matched only by a grep fallback is held in the partial-scope group with a fallback note
and downgrade reason. The rename never silently changes a candidate it could only match
lexically.

## The projected rename-preview-set object

Every governed preview also carries the frozen
[`RenamePreviewSet`](../../crates/aureline-navigation/src/target_model/mod.rs) it
projects: the candidate refs, the held (`blocked_refs`) set, conflict notes,
sparse/partial reasons, generated-scope notes, a `count_summary` of
changed/unresolved/generated/protected/skipped, and the apply posture. An invariant
checks that the projected object and the governed preview never disagree about what the
rename would change — so support, review, and AI consumers can reconstruct the rename
from either view.

## Replay and support

Every preview is metadata-only and serde-serializable, so search, graph, docs/help,
editor, AI, review, support, and CLI surfaces consume the same record. Because each
preview carries a stable id, the change-versus-held counts, per-group omission reasons,
conflict notes, and the apply gate, a support or debug packet can reconstruct **what
the rename would change, what it would not change, and why** — without any source body,
raw path, identifier, provider payload, URL, hostname, or credential. Refs are opaque
`aureline://` handles or repo-relative paths only.

## Frozen invariants

The corpus computes each invariant's `holds` flag from the builder's own output, so an
inconsistent change flips an invariant and fails CI:

- `rename_preview.candidate_grouping_disjoint`
- `rename_preview.counts_reconcile`
- `rename_preview.omissions_visible`
- `rename_preview.evidence_class_disclosed_no_grep_as_semantic`
- `rename_preview.conflict_notes_preserved`
- `rename_preview.inspect_before_mutate_enforced`
- `rename_preview.apply_posture_matches_groups`
- `rename_preview.partial_scope_truth`
- `rename_preview.preview_set_consistent`
- `rename_preview.consumers_preserve_truth`
- `rename_preview.corpus_covers_vocabulary`
- `rename_preview.replayable_support_answer`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/rename_preview.rs`](../../crates/aureline-navigation/tests/rename_preview.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it
equals the checked-in fixture, re-proves that every stored preview equals the builder's
own output, that the corpus is support-export safe, that every preview groups
candidates disjointly, reconciles change-versus-held counts, keeps omitted candidates
visible, enforces the inspect-before-mutate apply gate, and that every frozen invariant
holds — so a claimed rename surface cannot promote while a broad rename could flatten
into one apply action or drop its blocked, conflicting, generated, read-only, or
partial-scope candidates.

Regenerate the fixture after any builder change with:

```sh
cargo run -p aureline-navigation --example dump_rename_preview \
  > fixtures/navigation/governed_rename_preview/canonical_previews.json
```
