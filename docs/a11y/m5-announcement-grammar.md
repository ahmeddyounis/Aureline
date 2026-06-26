# M5 Live-Announcement Grammar

This document is the contract for the M5 live-announcement grammar that materializes
one announcement class per governed dynamic-event class. Where the frozen
dynamic-surface matrix governs *that* a live-announcement class exists and *which*
controlled vocabularies it carries, this catalog supplies the *concrete* grammar:
the stable message id, the placeholder-driven template, the polite/assertive
channel rule, the coalescing budget, the suppression rules, and the durable
fallback surface that shell, editor, terminal, notebook, data, review,
notifications, and help surfaces narrate M5 dynamic events through.

- Record kind: `m5_live_announcement_grammar_catalog`
- Schema: [`schemas/a11y/m5-announcement-grammar.schema.json`](../../schemas/a11y/m5-announcement-grammar.schema.json)
- Canonical support export: [`artifacts/a11y/m5-live-announcement-proof/support_export.json`](../../artifacts/a11y/m5-live-announcement-proof/support_export.json)
- Governance summary artifact: [`artifacts/a11y/m5-live-announcement-proof/live-announcement-proof.md`](../../artifacts/a11y/m5-live-announcement-proof/live-announcement-proof.md)
- Fixtures: [`fixtures/a11y/m5-announcements/`](../../fixtures/a11y/m5-announcements/)
- Producer: `aureline_shell::announcement_grammar::current_stable_m5_announcement_grammar_export`
- Headless emitter: `aureline_shell_m5_announcement_grammar`
- Frozen governing matrix: [`schemas/a11y/m5-dynamic-surface-a11y.schema.json`](../../schemas/a11y/m5-dynamic-surface-a11y.schema.json)

## Why this grammar exists

Screen-reader support on dynamic IDE surfaces fails when announcements are
improvised, overlong, duplicated, or detached from a durable surface the user can
revisit. Before this catalog each custom surface narrated state in its own ad hoc
prose: streaming logs spoke every repaint, background polls re-announced unchanged
truth, blockers and successes scrolled past in a transient live region with no
durable counterpart. The grammar makes narration a single governed packet: one
class per dynamic-event class, reused by every surface, by diagnostics, by support
exports, by docs/help, and by assistive-tech conformance packets.

## Governed event classes

The catalog carries one class for each governed dynamic-event class:

| Event class | Class | Channel | Coalescing | Durable fallback |
| --- | --- | --- | --- | --- |
| `mode_or_state_change` | `announcement:mode-or-state-change` | `polite` | `dedupe_same_meaning` | `status_detail` |
| `blocker_raised` | `announcement:blocker-raised` | `assertive` | `dedupe_same_meaning` | `banner_detail` |
| `progress_milestone` | `announcement:progress-milestone` | `polite` | `start_and_terminal_only` | `run_header` |
| `selection_or_context_change` | `announcement:selection-or-context-change` | `polite` | `last_meaning_wins_with_count` | `selection_summary` |
| `success_with_recovery` | `announcement:success-with-recovery` | `polite` | `dedupe_same_meaning` | `activity_row` |
| `degraded_or_stale_truth` | `announcement:degraded-or-stale-truth` | `polite` | `dedupe_same_meaning` | `notification_center_entry` |

## What each class binds

Each `grammar_class` binds a stable `class_id` to:

- **A stable message id and placeholder-driven template** — the `message_template`
  carries a `message_id` (prefixed `announcement.`), a single `template` string
  with `{placeholder}` insertion points, and one declared `placeholder` per token.
  The template — not the call site — owns the sentence shape, so narration is never
  built from concatenated fragments. The validator rejects any template whose
  `{...}` tokens disagree with its declared placeholders.
- **A live-region channel** — `channel` is `polite`, `assertive`, or `silent`. Only
  the blocker class may interrupt with an `assertive` live region; every other class
  stays `polite` so the live region is never spammed by urgency.
- **Required runtime fields** — `required_fields` lists exactly the placeholders a
  caller must supply, kept in lockstep with the required placeholders in the
  template.
- **A coalescing budget** — `coalescing_budget` pairs the matrix-owned `strategy`
  (never `none`) with hard caps: `max_announcements_per_window`, `window_seconds`,
  and `min_interval_ms`. The budget bounds how often the live region speaks so
  repeated polls, streaming updates, and background refreshes cannot flood it.
- **Suppression rules** — one or more `suppression_rules` (e.g.
  `suppress_unchanged_meaning`, `suppress_low_value_progress_midpoints`,
  `suppress_background_refresh_when_unfocused`) so duplicate or low-value narration
  is dropped rather than spoken.
- **A durable fallback surface** — `fallback_durability` (matrix-owned) plus a
  `durable_fallback` that names the surface (`activity_row`, `run_header`,
  `patch_review_header`, `banner_detail`, `selection_summary`,
  `notification_center_entry`, or `status_detail`), its stable `surface_ref`, and a
  `reopenable` flag. Every announcement has a durable UI counterpart the user can
  reopen instead of relying on ephemeral narration alone.

## Controlled vocabulary reuse

The controlled state vocabularies — `announcement_politeness`,
`coalescing_strategy`, and `fallback_durability` — are reused verbatim from the
frozen dynamic-surface matrix through the `shared_vocabulary_set` block, which must
match the matrix's canonical token lists. The grammar-shaped vocabularies this lane
adds — `event_class`, `durable_fallback_surface`, `value_kind`, and
`suppression_rule` — are frozen in the `grammar_vocabulary_set` block. No surface
mints a parallel synonym for a governed announcement state.

## Auto-narrowing on degraded bridge or stale proof

A class whose assistive-tech proof has gone stale narrows its qualification (for
example Stable to Beta) while keeping its message template, channel, budget, and
durable fallback intact and carrying a `proof_stale` downgrade trigger. A class
whose OS live region is unavailable narrows (for example Stable to Preview), shifts
its delivery to `durable_surface_only`, and carries a `bridge_unavailable` trigger —
the announcement still has a durable counterpart even with no live region. The
`proof_stale_narrowed.json` and `live_region_unavailable_narrowed.json` fixtures
exercise both paths: the narrowing is always a disclosed claim change, never a
hidden class.

## Consumers

`shell`, `editor`, `terminal`, `notebook`, `data_grid`, and `review` surfaces
narrate their dynamic events through these classes; `notifications` route durable
announcements through them; diagnostics, support exports, docs/help, and
assistive-tech conformance packets reuse the same grammar. The
`consumer_projection` block records that every one of those consumers narrates
through the grammar rather than improvising per-surface prose.

## Regenerating the catalog

The seed builders in `aureline_shell::announcement_grammar` are the single producer
of the checked-in support export and fixtures. Regenerate with the headless
emitter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- support-export \
  > artifacts/a11y/m5-live-announcement-proof/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- markdown \
  > artifacts/a11y/m5-live-announcement-proof/live-announcement-proof.md
cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- fixture-proof-stale-narrowed \
  > fixtures/a11y/m5-announcements/proof_stale_narrowed.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- fixture-live-region-unavailable-narrowed \
  > fixtures/a11y/m5-announcements/live_region_unavailable_narrowed.json
```

The `checked_support_export_matches_seed` test fails if the checked-in export drifts
from the seed builder, so the artifact, the fixtures, and the in-code grammar stay
in lockstep.
