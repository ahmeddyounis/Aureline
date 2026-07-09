# M5 learning educational-AI continuity controls

Status: implemented. This contract governs the **degraded and disconnected states** of every
reusable learning component frozen in the
[M5 learning component matrix](m5_learning_component_matrix.md) — the learning-mode toggle, the
tip card, the guided exercise step, the glossary chip/card, the safe explanation banner, and the
progress marker — across the claimed M5 guided-teaching surfaces. It closes the acceptance-criteria
gap that remains once a remote enrichment is missing, a docs pack is stale, a citation is
unavailable, the network is gone, or a pack was never installed: a learner must be able to tell,
before acting, whether they are looking at live, cached, local-only, offline, stale, uncited, or
not-installed learning content, and educational AI must never mutate live state without crossing
the ordinary preview / approval path.

- Crate module:
  `crates/aureline-learning/src/ship_educational_ai_boundaries_no_hidden_apply_safeguards_and_offline_local_only_or_cached_pack_continuity_across_claimed_m5_guided_teaching_flows`
- Boundary schema:
  [`schemas/ui/m5-learning-educational-ai-continuity-controls.schema.json`](../../schemas/ui/m5-learning-educational-ai-continuity-controls.schema.json)
- Checked support export:
  `artifacts/release/m5-learning-educational-ai-continuity-proof/support_export.json`
  (plus `matrix.csv` and `summary.md`)
- Scenario fixtures:
  `fixtures/ui/m5-learning-educational-ai-continuity-controls/`
- Headless emitter:
  `cargo run -p aureline-learning --bin aureline_learning_m5_educational_ai_continuity_primitive -- <support-export|report|csv|fixture-*|validate>`

## Reused frozen vocabulary

This lane never invents a parallel learning grammar. It reuses the frozen matrix enums verbatim:
component family, the one controlled disposition vocabulary, learning-mode scope, required labels,
surface family, deployment line, consumer surface, accessibility route, and downgrade triggers. It
mints new vocabulary only for what the matrix left implicit about degraded and disconnected
states: the controlled continuity state, the derived data-trust class, the derived
next-safe-action, the educational-AI apply posture and its derived apply disposition, the subject
kind, the reachable source kind, and the keyboard-complete safe verbs.

## Degraded component

Each component names one governed component family, the subject it teaches (`subject_kind`,
`subject_label`), a **preserved subject summary** (`subject_summary_note` — the last-known summary
shown even when full enrichment cannot be fetched), its **exact cited source reference**
(`cited_source_ref` — the one stable cited source `OpenSource` lands on), its **stable component
identity** (`stable_component_ref`), its learning scope, and its governed **continuity state**.

Its **data-trust class** and its **next-safe-action** are both *derived* from the continuity state
by `resolve_continuity`, never asserted:

| continuity state | data-trust class | live? | source reachable? | next-safe-action |
| --- | --- | --- | --- | --- |
| `live` | `live_enriched` | yes | yes | `proceed_in_learning` |
| `cached` | `cached_pack` | no | yes (fallback) | `refresh_enrichment` |
| `local_only` | `local_only_bounded` | no | yes (fallback) | `continue_local_only` |
| `offline` | `offline_held` | no | yes (fallback) | `retry_when_online` |
| `stale_pack` | `stale_unverified` | no | yes (fallback) | `update_docs_pack` |
| `citation_unavailable` | `uncited_withheld` | no | no (stops routing) | `show_uncited_explicitly` |
| `not_installed` | `not_installed_unavailable` | no | no (stops routing) | `install_to_enable` |

A cached, local-only, offline, or stale component can therefore never read as live. Any component
that is not live carries an explicit **continuity-state explanation** (`state_explanation_note`),
and every component carries an explicit **next-safe-action copy** (`next_safe_action_note`), so a
degraded state and what to do about it are always visible before an action.

## Offline / local-only / cached-pack continuity

Learning stays useful offline. A component whose enrichment is degraded but whose cited source is
still reachable — `cached`, `local_only`, `offline`, `stale_pack` — names a **source fallback**
(`source_fallback_note`) and offers a resolvable cited source (the `open_source` verb plus a
`source_kind` other than `no_source`), so the learner can still open the cited file, symbol, docs
page, command, or sandbox target. A component whose citation is unavailable or whose pack is not
installed degrades to an **explicit** uncited / not-installed state: it carries no cited source
(`cited_source_ref` is empty, `source_kind` is `no_source`, the `open_source` verb is withheld) and
stops routing to a source it does not have. The exact `cached`, `local_only`, and `not_installed`
truth is always preserved.

## No-hidden-apply safeguards

The educational-AI **apply posture** is an independent axis. Its **apply disposition** and its
**live-mutation flag** are *derived* from the posture by `resolve_apply`, never asserted:

| apply posture | apply disposition | offers live mutation? | requires preview/approval? |
| --- | --- | --- | --- |
| `explain_only` | `no_mutation` | no | n/a |
| `sandboxed_practice` | `sandbox_mutation_only` | no | n/a |
| `preview_then_approve` | `preview_approval_required` | yes | yes |
| `apply_blocked` | `mutation_unavailable` | no | n/a |

The only posture that offers a live mutation is `preview_then_approve`, and it always requires the
ordinary preview / approval crossing — so **educational AI can never mutate live state without it**.
Every component carries an explicit explain-versus-do boundary note (`apply_boundary_note`), and the
hard invariant `mutates_live_without_preview_approval` MUST be `false`.

## Hard invariants

Every component reuses the four matrix invariants and adds the no-hidden-apply one; all MUST be
`false`:

- `masks_privacy_or_offline_state`
- `hides_citation_source`
- `invents_alternate_state_label`
- `implies_hidden_apply_or_mutation`
- `mutates_live_without_preview_approval`

## Coverage

The canonical packet covers all seven continuity states, all six component families, and all four
apply postures. Validation fails closed on any missing coverage, any misrepresented derived truth,
any missing note, any withheld mandatory `explain` verb or mandatory label, any accessibility gap,
or any raw boundary material in the export.

## Acceptance criteria

- **Educational AI cannot mutate live state without crossing the ordinary preview/approval path.**
  The apply disposition and live-mutation flag are derived from the posture; only
  `preview_then_approve` offers a live mutation and it always requires approval; the
  `mutates_live_without_preview_approval` invariant is enforced `false`.
- **Learning components stay useful offline while preserving exact `cached`, `local-only`, or
  `not installed` truth.** Degraded-but-reachable states keep a resolvable cited source fallback;
  citation-unavailable and not-installed states degrade to explicit states and stop routing; the
  continuity state, trust class, and next-safe-action are always explicit before an action.
