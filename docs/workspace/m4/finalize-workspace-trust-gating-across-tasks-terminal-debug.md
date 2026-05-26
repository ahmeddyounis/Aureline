# Trust-gating lineage — contract

This document describes the trust-gating lineage record: the workspace's
governed, export-safe projection that finalizes workspace-trust gating
across tasks, terminal, debug, AI apply, and privileged extensions.

The record is the single artifact every consuming surface (workspace
trust-gating status, grant-scope review sheet, support export, Help/About,
headless CLI) ingests instead of cloning status text.

## Input

The projection ingests a live
[`TrustGatingInputs`](../../../crates/aureline-workspace/src/trust_gating_lineage/mod.rs)
envelope verbatim. The envelope carries the captured workspace trust
posture plus one
[`TrustSurfaceObservation`](../../../crates/aureline-workspace/src/trust_gating_lineage/mod.rs)
per privileged surface (tasks, terminal, debug, AI apply, privileged
extensions). Each surface row records the declared gate decision,
silent-execution posture, override route, disclosure ids, whether the
surface touches a credential store, and the support-export projection.

For determinism and replay, the projection accepts the same envelope
shape the fixtures and the headless emitter consume.

## What the record proves

The contract claims the stable line is anchored on, specialised to
workspace trust:

- **Surface coverage truth.** Every privileged workspace surface gated
  by workspace trust is bound to one closed surface kind (`tasks`,
  `terminal`, `debug`, `ai_apply`, `privileged_extension`), and the
  corpus seeds at least one row per kind so the trust gate is
  observable on every surface that can mutate or exfiltrate workspace
  state.
- **Gate-decision truth.** Every surface declares one closed
  [`GateDecisionClass`](../../../crates/aureline-workspace/src/trust_gating_lineage/mod.rs);
  the projection re-derives the worst-case decision from the captured
  workspace trust posture and surfaces both `declared_gate_decision`
  and `derived_gate_decision`. A `restricted` workspace cannot ship
  `allow_unconditional` or `allow_after_explicit_grant`; a
  `pending_evaluation` workspace cannot ship anything other than a
  blocking / read-only decision.
- **No-silent-execution honesty.** Surfaces that allow execution after
  an explicit grant must require an explicit user action and reference
  a disclosure id, so terminals, tasks, debuggers, AI apply, and
  privileged extensions never resume silently. Read-only surfaces must
  declare the `read_only_no_mutation` posture.
- **Override-route honesty.** Every non-`none` override route must
  reference both an override action id and an override disclosure id,
  so the user can inspect what an override unlocks before it commits.
- **Trust-review hook honesty.** A controlled set of pre-execution
  inspection / repair hooks (`inspect_trust_grant`, `review_grant_scope`,
  `compare_workspace_trust`, `rollback_grant`, `export`, `repair`) is
  reachable so any destructive grant or privileged execution can be
  reviewed before it fires.
- **Support-export honesty.** Each surface's support-export projection
  preserves the surface kind, gate decision, override route,
  silent-execution posture, and disclosure id while excluding raw
  secrets, approval tickets, delegated credentials, and live authority
  handles. Credential-touching surfaces must declare a non-`local_only`
  posture so support bundles can preserve the gating decision.

In addition the record carries the producer ref, the schema version,
the capture timestamp, and an integrity hash so import / replay
surfaces can pin the source producer before applying.

## Closed vocabularies

| Field                         | Tokens                                                                                                                                                                                                  |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `trust_surface_kind`          | `tasks`, `terminal`, `debug`, `ai_apply`, `privileged_extension`                                                                                                                                        |
| `workspace_trust_posture`     | `trusted`, `restricted`, `pending_evaluation`                                                                                                                                                           |
| `gate_decision_class`         | `allow_unconditional`, `allow_after_explicit_grant`, `allow_read_only`, `block_pending_trust_decision`, `block_until_repair`                                                                            |
| `silent_execution_posture`    | `cannot_fire_silently`, `explicit_user_action_required`, `read_only_no_mutation`                                                                                                                        |
| `override_route_class`        | `none`, `disclosed_one_time`, `disclosed_session`, `disclosed_with_audit`                                                                                                                               |
| `support_export_posture`      | `local_only`, `metadata_safe_export`, `held_record`                                                                                                                                                     |
| `inspection_hook_class`       | `inspect_trust_grant`, `review_grant_scope`, `compare_workspace_trust`, `rollback_grant`, `export`, `repair`                                                                                            |

## Narrow reasons

When a claim cannot be proven on the captured posture the record
auto-narrows below Stable with a named reason.

| Narrow reason                                     | Fires when                                                                                                                       |
| ------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `corpus_empty`                                    | The envelope carried no surface observations                                                                                     |
| `required_trust_surface_missing`                  | The corpus omitted at least one of the five required privileged surface kinds                                                    |
| `restricted_workspace_allows_unconditional`       | A `restricted` workspace declared an execution-allowing gate on at least one surface                                             |
| `pending_workspace_allows_execution`              | A `pending_evaluation` workspace declared an execution-allowing gate on at least one surface                                     |
| `unconditional_allow_without_trusted_posture`     | A surface declared `allow_unconditional` while the workspace trust posture was not `trusted`                                     |
| `silent_grant_without_disclosure`                 | A surface declared `allow_after_explicit_grant` but did not require explicit user action or omitted a disclosure id              |
| `override_route_undisclosed`                      | A surface declared a non-`none` override route but did not reference an override action id or an override disclosure id          |
| `read_only_missing_posture`                       | A surface declared `allow_read_only` but did not declare the `read_only_no_mutation` silent-execution posture                    |
| `inspection_hook_unavailable`                     | A required pre-execution inspection / repair hook was unavailable                                                                |
| `support_export_fields_dropped`                   | A surface's support-export projection dropped one of the required gating fields                                                  |
| `support_export_redaction_unsafe`                 | A surface declared `raw_secrets_excluded = false`, `approval_tickets_excluded = false`, `delegated_credentials_excluded = false`, or `live_authority_handles_excluded = false` |
| `support_export_posture_unsafe`                   | A credential-touching surface declared `local_only` support export                                                               |
| `producer_attribution_incomplete`                 | Producer attribution fields were empty (producer ref / captured-at)                                                              |
| `lineage_export_unsafe`                           | Workspace ref or corpus ref was empty (would break support export)                                                               |

## Inspection hooks

A destructive grant, privileged execution, or repair never fires
without an inspection hook the user can reach first.

| Hook class                | Default action id                                | Purpose                                                                                                                                |
| ------------------------- | ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------- |
| `inspect_trust_grant`     | `trust_gating.inspect_trust_grant`              | Opens the trust-grant inspector with the workspace posture, recent grant history, and which privileged surfaces are gated              |
| `review_grant_scope`      | `trust_gating.review_grant_scope`               | Opens the grant-scope review sheet so any privileged execution can be reviewed before it fires                                         |
| `compare_workspace_trust` | `trust_gating.compare_workspace_trust`          | Produces a reviewable diff between the prior trust posture and the current grant                                                       |
| `rollback_grant`          | `trust_gating.rollback_grant`                   | Captures a one-step rollback so the user can revert a trust grant if a privileged surface misbehaves                                   |
| `export`                  | `trust_gating.export`                           | Exports the lineage record (support-safe, no raw secrets, approval tickets, or delegated credentials)                                  |
| `repair`                  | `trust_gating.repair`                           | Opens the repair sheet for a restricted workspace and surfaces the manual remediation steps                                            |

## Replay gate

Every fixture under
[`/fixtures/workspace/m4/trust_gating_lineage/`](../../../fixtures/workspace/m4/trust_gating_lineage/)
carries the posture inputs and the expected projected record. The
replay gate at
[`/crates/aureline-workspace/tests/trust_gating_lineage_replay.rs`](../../../crates/aureline-workspace/tests/trust_gating_lineage_replay.rs)
re-projects each input and asserts the result equals the checked-in
`expected`, so the projection cannot drift from the canonical record
without failing CI. The gate also asserts each fixture is support-export
safe and that the corpus covers every controlled workspace-trust
posture plus a narrowed-below-Stable posture.
