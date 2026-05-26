# Ship Restricted-Mode UX Completion and Explainability on Claimed Stable Rows — proof packet

Reviewer-facing proof packet for the restricted-mode UX lane: every
user-facing restricted-mode surface (status bar, editor chrome,
command palette, action menu, Help/About, support export) is bound to
one closed restriction reason and escape path, ships a named
explanation id, and declares a claimed stability tier that the
projection re-derives from the captured workspace posture. A read-only
claim that exposes mutation, a `stable_full` claim in a restricted or
pending workspace, an undisclosed `grant_trust` escape, or a missing
accessibility posture narrows the record below Stable with a named
reason. This packet is the stable-line anchor for this lane;
dashboards, docs, Help/About surfaces, and support exports should
ingest the typed sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/restricted_mode_ux_lineage/`](../../../crates/aureline-workspace/src/restricted_mode_ux_lineage/)
- Schema:
  [`/schemas/workspace/restricted_mode_ux_lineage.schema.json`](../../../schemas/workspace/restricted_mode_ux_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_restricted_mode_ux_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_restricted_mode_ux_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/restricted_mode_ux_lineage/`](../../../fixtures/workspace/m4/restricted_mode_ux_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/restricted_mode_ux_lineage_replay.rs`](../../../crates/aureline-workspace/tests/restricted_mode_ux_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/ship-restricted-mode-ux-completion-and-explainability-on.md`](../../../docs/workspace/m4/ship-restricted-mode-ux-completion-and-explainability-on.md)
- Typed consumer: `aureline_workspace::project_restricted_mode_ux_lineage`

## What this packet proves

1. **Surface coverage truth.** Each record carries a
   [`surface_coverage`](../../../schemas/workspace/restricted_mode_ux_lineage.schema.json)
   row per restricted-mode UX surface declaring one closed
   `restricted_mode_surface_kind` (`status_bar`, `editor_chrome`,
   `command_palette`, `action_menu`, `help_about`, `support_export`).
   A corpus missing any required surface narrows below Stable with
   `required_restricted_surface_missing`. Worked example:
   [`restricted_read_only_stable.json`](../../../fixtures/workspace/m4/restricted_mode_ux_lineage/restricted_read_only_stable.json).

2. **Explainability truth.** Every restricted-mode surface declares a
   named `restriction_reason_class` and references a stable
   `explanation_id`. A restricted or pending workspace that omits the
   explanation narrows with `explanation_missing`.

3. **Escape-path honesty.** Every surface declares a named
   `escape_path_class`. A `grant_trust` escape must reference both an
   action id and a disclosure id (mismatches narrow with
   `grant_trust_escape_undisclosed`); any non-`stay_read_only` escape
   must reference an action id (mismatches narrow with
   `escape_path_action_missing`).

4. **Read-only affordance truth.** Surfaces that claim
   `stable_read_only` declare only read-only-safe affordances
   (`inspect_only`, `copy_to_clipboard`, `navigate_only`,
   `view_diff_only`, `blocked_with_explanation`,
   `allow_read_only_no_mutation`). A read-only claim that exposes a
   mutation affordance narrows with
   `read_only_claim_exposes_mutation`; a surface with no affordances
   narrows with `affordances_empty`.

5. **Claimed-tier truth.** The projection re-derives the worst-case
   tier from the captured workspace posture and surfaces both
   `declared_tier` and `derived_tier`. A `stable_full` claim in a
   restricted or pending workspace, or a declared tier that does not
   match the derived tier, narrows with
   `claimed_full_in_restricted_posture`. Worked examples:
   [`pending_read_only_stable.json`](../../../fixtures/workspace/m4/restricted_mode_ux_lineage/pending_read_only_stable.json),
   [`trusted_stable_full.json`](../../../fixtures/workspace/m4/restricted_mode_ux_lineage/trusted_stable_full.json).

6. **Accessibility truth.** Every surface declares the five required
   accessibility postures (`keyboard_only`, `screen_reader`,
   `ime_grapheme_bidi`, `zoom_high_contrast`, `reduced_motion`). A
   surface missing any required posture narrows with
   `accessibility_posture_missing`.

7. **Support-export honesty.** Each surface's support-export
   projection must preserve `surface_kind`, `restriction_reason`,
   `explanation_id`, `escape_path`, `claimed_tier`, and
   `accessibility_postures`, redact raw secrets, approval tickets,
   delegated credentials, and live authority handles, and (for
   credential-touching surfaces) declare a non-`local_only` posture.
   Dropping a field narrows with `support_export_fields_dropped`;
   raising raw material narrows with `support_export_redaction_unsafe`;
   a credential-touching surface shipping `local_only` narrows with
   `support_export_posture_unsafe`.

8. **Inspection precedes destructive grants.** A controlled inspection
   / repair hook table must be available before any destructive grant
   or tier-widening commit. The required classes are
   `inspect_restriction`, `review_escape_path`, `compare_unrestricted`,
   `rollback_grant`, `export`, and `repair`. A missing hook narrows
   with `inspection_hook_unavailable`. Worked example:
   [`missing_review_escape_path_hook_narrowed.json`](../../../fixtures/workspace/m4/restricted_mode_ux_lineage/missing_review_escape_path_hook_narrowed.json).

9. **Producer attribution is pinnable for replay.** Each record
   carries the producer ref, the schema version, the capture
   timestamp, and an integrity hash derived from the input surface
   identities so replay and support pipelines can pin the source
   before applying. Incomplete attribution narrows with
   `producer_attribution_incomplete`.

10. **Lineage and export stay honest.** Every record sets
    `raw_payload_excluded = true` and carries only opaque refs to the
    source workspace, corpus, and producer. An empty workspace or
    corpus ref narrows with `lineage_export_unsafe`.

11. **The record is replay-gated.** The replay gate re-projects each
    fixture and asserts it equals the checked-in `expected`, so the
    projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                       | Workspace posture       | Tier claimed across surfaces | Qualification           | Proves                                                                                       |
| --------------------------------------------- | ----------------------- | ---------------------------- | ----------------------- | -------------------------------------------------------------------------------------------- |
| `restricted_read_only_stable`                 | `restricted`            | `stable_read_only`           | `stable`                | A restricted workspace can claim Stable per surface in a read-only tier with full explainability |
| `pending_read_only_stable`                    | `pending_evaluation`    | `stable_read_only`           | `stable`                | A pending workspace stays read-only-tier across the restricted-mode UX surfaces              |
| `trusted_stable_full`                         | `trusted`               | `stable_full`                | `stable`                | A trusted workspace can dormant-claim Stable across surfaces without exposing a restriction  |
| `missing_review_escape_path_hook_narrowed`    | `restricted`            | `stable_read_only`           | `narrowed_below_stable` | A missing `review_escape_path` hook narrows the record below Stable                          |

## How to verify

```sh
# Unit + replay gate for the restricted-mode UX lineage projection.
cargo test -p aureline-workspace --lib restricted_mode_ux_lineage
cargo test -p aureline-workspace --test restricted_mode_ux_lineage_replay

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_restricted_mode_ux_lineage -- --lines \
  fixtures/workspace/m4/restricted_mode_ux_lineage/restricted_read_only_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: surfaces that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public scope
is widened from this row.
