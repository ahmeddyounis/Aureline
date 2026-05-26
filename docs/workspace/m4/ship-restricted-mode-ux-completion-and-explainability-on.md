# Restricted-mode UX lineage — contract

This document describes the restricted-mode UX lineage record: the
workspace's governed, export-safe projection that finalizes the
user-facing restricted-mode experience and the explainability surfaces
that ride on top of workspace-trust gating.

Where the trust-gating lineage proves *which privileged surfaces are
gated and why*, this record proves *what the user sees when they land
in a restricted (or pending) workspace*: which UX surfaces show the
restriction, which named explanation each one carries, which escape
paths are offered without silent commitments, which read-only
affordances stay reachable, which accessibility postures are honored,
and which stability tier each surface is allowed to claim on the
release branch.

The record is the single artifact every consuming surface (workspace
restricted-mode status, escape-path review sheet, support export,
Help/About, headless CLI) ingests instead of cloning status text.

## Input

The projection ingests a live
[`RestrictedModeUxInputs`](../../../crates/aureline-workspace/src/restricted_mode_ux_lineage/mod.rs)
envelope verbatim. The envelope carries the captured workspace
restricted-mode posture plus one
[`RestrictedModeSurfaceObservation`](../../../crates/aureline-workspace/src/restricted_mode_ux_lineage/mod.rs)
per restricted-mode UX surface (status bar, editor chrome, command
palette, action menu, Help/About, support export). Each surface row
records the restriction reason, explanation id, escape path,
escape-action / escape-disclosure ids, declared affordances, claimed
tier, accessibility postures, and the support-export projection.

For determinism and replay, the projection accepts the same envelope
shape the fixtures and the headless emitter consume.

## What the record proves

- **Surface coverage truth.** Every restricted-mode UX surface that
  ships a restriction message is bound to one closed surface kind
  (`status_bar`, `editor_chrome`, `command_palette`, `action_menu`,
  `help_about`, `support_export`), and the corpus seeds one row per
  kind so the user never lands on a restricted workspace surface that
  hides the restriction entirely.
- **Explainability truth.** Every surface declares a named restriction
  reason and references a stable `explanation_id` so the user can
  pivot to the restriction explanation from any surface without
  re-routing through the trust grant flow first.
- **Escape-path honesty.** Every surface declares one closed escape
  path (`grant_trust`, `repair_workspace`, `leave_workspace`,
  `stay_read_only`, `contact_support`). A `grant_trust` escape must
  reference an explicit action id and a disclosure id so it cannot
  silently commit to widening trust. Non-`stay_read_only` escapes must
  reference an action id.
- **Read-only affordance truth.** Surfaces that claim
  `stable_read_only` declare only inspect-class affordances and
  explicitly exclude mutation, execution, and exfiltration affordances.
  The projection re-derives the affordance posture so a read-only
  claim that exposes mutation narrows.
- **Claimed-tier truth.** Each surface declares one closed
  `claimed_stable_tier`; the projection re-derives the worst-case tier
  from the captured restriction posture so a `stable_full` claim
  cannot ride out of a restricted workspace.
- **Accessibility truth.** Every surface declares whether it ships the
  five required accessibility postures (keyboard, screen reader, IME /
  grapheme / bidi, zoom / high contrast, reduced motion). A surface
  missing any required posture narrows below Stable.
- **Pre-action inspection-hook honesty.** A controlled set of
  pre-action inspection / repair hooks
  (`inspect_restriction`, `review_escape_path`,
  `compare_unrestricted`, `rollback_grant`, `export`, `repair`) is
  reachable so destructive grants and tier-widening commits stay
  reviewable.
- **Support-export honesty.** Each surface's support-export projection
  preserves the surface kind, restriction reason, explanation id,
  escape path, claimed tier, and accessibility posture while excluding
  raw secrets, approval tickets, delegated credentials, and live
  authority handles. Credential-touching surfaces must declare a
  non-`local_only` posture so support bundles can preserve the
  restricted-mode state.

In addition the record carries the producer ref, the schema version,
the capture timestamp, and an integrity hash so import / replay
surfaces can pin the source producer before applying.

## Closed vocabularies

| Field                            | Tokens                                                                                                                                  |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `restricted_mode_surface_kind`   | `status_bar`, `editor_chrome`, `command_palette`, `action_menu`, `help_about`, `support_export`                                         |
| `restricted_mode_posture`        | `trusted`, `restricted`, `pending_evaluation`                                                                                           |
| `restriction_reason_class`       | `workspace_restricted`, `workspace_pending_evaluation`, `surface_read_only`, `credential_store_restricted`, `policy_block`, `post_entry_staging_restricted` |
| `escape_path_class`              | `grant_trust`, `repair_workspace`, `leave_workspace`, `stay_read_only`, `contact_support`                                               |
| `restricted_affordance_class`    | `inspect_only`, `copy_to_clipboard`, `navigate_only`, `view_diff_only`, `blocked_with_explanation`, `allow_read_only_no_mutation`       |
| `claimed_stable_tier`            | `stable_full`, `stable_read_only`, `narrowed_below_stable`                                                                              |
| `accessibility_posture_class`    | `keyboard_only`, `screen_reader`, `ime_grapheme_bidi`, `zoom_high_contrast`, `reduced_motion`                                            |
| `support_export_posture`         | `local_only`, `metadata_safe_export`, `held_record`                                                                                     |
| `inspection_hook_class`          | `inspect_restriction`, `review_escape_path`, `compare_unrestricted`, `rollback_grant`, `export`, `repair`                                |

## Narrow reasons

When a claim cannot be proven on the captured posture the record
auto-narrows below Stable with a named reason.

| Narrow reason                          | Fires when                                                                                                                       |
| -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `corpus_empty`                         | The envelope carried no surface observations                                                                                     |
| `required_restricted_surface_missing`  | The corpus omitted at least one of the six required restricted-mode UX surface kinds                                             |
| `explanation_missing`                  | A surface in a restricted or pending workspace did not ship an explanation id                                                    |
| `grant_trust_escape_undisclosed`       | A surface declared a `grant_trust` escape path but omitted the escape action id or disclosure id                                 |
| `escape_path_action_missing`           | A surface declared a non-`stay_read_only` escape path but omitted its escape action id                                           |
| `read_only_claim_exposes_mutation`     | A surface claimed `stable_read_only` but exposed an affordance that is not read-only-safe                                        |
| `claimed_full_in_restricted_posture`   | A surface claimed `stable_full` in a restricted or pending workspace, or the declared tier did not match the derived worst-case tier |
| `affordances_empty`                    | A surface declared no affordances at all                                                                                         |
| `accessibility_posture_missing`        | A surface is missing at least one required accessibility posture                                                                 |
| `inspection_hook_unavailable`          | A required pre-action inspection / repair hook was unavailable                                                                   |
| `support_export_fields_dropped`        | A surface's support-export projection dropped one of the required restricted-mode fields                                         |
| `support_export_redaction_unsafe`      | A surface declared `raw_secrets_excluded = false`, `approval_tickets_excluded = false`, `delegated_credentials_excluded = false`, or `live_authority_handles_excluded = false` |
| `support_export_posture_unsafe`        | A credential-touching surface declared `local_only` support export                                                               |
| `producer_attribution_incomplete`      | Producer attribution fields were empty (producer ref / captured-at)                                                              |
| `lineage_export_unsafe`                | Workspace ref or corpus ref was empty (would break support export)                                                               |

## Inspection hooks

A destructive grant, tier-widening commit, or repair never fires
without an inspection hook the user can reach first.

| Hook class                | Default action id                                  | Purpose                                                                                                                        |
| ------------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `inspect_restriction`     | `restricted_mode.inspect_restriction`             | Opens the restriction inspector with the captured restriction reason, the surface affected, and the current escape-path offer  |
| `review_escape_path`      | `restricted_mode.review_escape_path`              | Opens the escape-path review sheet so any tier-widening or trust-grant commit can be reviewed before it fires                  |
| `compare_unrestricted`    | `restricted_mode.compare_unrestricted`            | Renders the surface affordance diff between the restricted-mode posture and the unrestricted baseline                          |
| `rollback_grant`          | `restricted_mode.rollback_grant`                  | Captures a one-step rollback so the user can revert a trust grant if a restricted-mode surface widens unexpectedly             |
| `export`                  | `restricted_mode.export`                          | Exports the lineage record (support-safe, no raw secrets, approval tickets, or delegated credentials)                          |
| `repair`                  | `restricted_mode.repair`                          | Opens the repair sheet for a restricted workspace and surfaces the manual remediation steps                                    |

## Replay gate

Every fixture under
[`/fixtures/workspace/m4/restricted_mode_ux_lineage/`](../../../fixtures/workspace/m4/restricted_mode_ux_lineage/)
carries the posture inputs and the expected projected record. The
replay gate at
[`/crates/aureline-workspace/tests/restricted_mode_ux_lineage_replay.rs`](../../../crates/aureline-workspace/tests/restricted_mode_ux_lineage_replay.rs)
re-projects each input and asserts the result equals the checked-in
`expected`, so the projection cannot drift from the canonical record
without failing CI. The gate also asserts each fixture is
support-export safe and that the corpus covers Stable plus the three
controlled workspace postures (Trusted / Restricted /
PendingEvaluation) and a narrowed-below-Stable posture.
