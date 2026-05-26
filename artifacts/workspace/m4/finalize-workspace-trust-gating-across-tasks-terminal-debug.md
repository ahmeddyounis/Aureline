# Finalize Workspace-Trust Gating Across Tasks, Terminal, Debug, AI Apply, and Privileged Extensions — proof packet

Reviewer-facing proof packet for the workspace-trust gating lane: every
privileged workspace surface (tasks, terminal, debug, AI apply,
privileged extensions) is bound to one closed gate decision drawn from
the workspace trust posture, and no privileged execution can fire
silently or bypass the disclosure that the workspace trust grant
already required. This packet is the stable-line anchor for this lane;
dashboards, docs, Help/About surfaces, and support exports should
ingest the typed sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/trust_gating_lineage/`](../../../crates/aureline-workspace/src/trust_gating_lineage/)
- Schema:
  [`/schemas/workspace/trust_gating_lineage.schema.json`](../../../schemas/workspace/trust_gating_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_trust_gating_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_trust_gating_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/trust_gating_lineage/`](../../../fixtures/workspace/m4/trust_gating_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/trust_gating_lineage_replay.rs`](../../../crates/aureline-workspace/tests/trust_gating_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/finalize-workspace-trust-gating-across-tasks-terminal-debug.md`](../../../docs/workspace/m4/finalize-workspace-trust-gating-across-tasks-terminal-debug.md)
- Typed consumer: `aureline_workspace::project_trust_gating_lineage`

## What this packet proves

1. **Surface coverage truth.** Each record carries a
   [`surface_coverage`](../../../schemas/workspace/trust_gating_lineage.schema.json)
   row per privileged surface declaring one closed
   `trust_surface_kind` (`tasks`, `terminal`, `debug`, `ai_apply`,
   `privileged_extension`). A corpus missing any required surface
   narrows below Stable with `required_trust_surface_missing`. Worked
   example:
   [`trusted_grant_stable.json`](../../../fixtures/workspace/m4/trust_gating_lineage/trusted_grant_stable.json).

2. **Gate-decision truth.** Every surface declares one closed
   `gate_decision_class` drawn from the vocabulary
   (`allow_unconditional`, `allow_after_explicit_grant`,
   `allow_read_only`, `block_pending_trust_decision`,
   `block_until_repair`). The projection re-derives the worst-case
   decision from the captured workspace trust posture and surfaces
   both `declared_gate_decision` and `derived_gate_decision`. A
   `restricted` workspace declaring an execution-allowing gate narrows
   with `restricted_workspace_allows_unconditional`; a
   `pending_evaluation` workspace narrows with
   `pending_workspace_allows_execution`; a non-trusted workspace
   declaring `allow_unconditional` narrows with
   `unconditional_allow_without_trusted_posture`. Worked examples:
   [`restricted_blocked_stable.json`](../../../fixtures/workspace/m4/trust_gating_lineage/restricted_blocked_stable.json),
   [`pending_blocked_stable.json`](../../../fixtures/workspace/m4/trust_gating_lineage/pending_blocked_stable.json),
   [`restricted_read_only_stable.json`](../../../fixtures/workspace/m4/trust_gating_lineage/restricted_read_only_stable.json).

3. **No-silent-execution honesty.** Surfaces that allow execution
   after an explicit grant must require an explicit user action and
   reference a disclosure id; read-only surfaces must declare the
   `read_only_no_mutation` posture. Mismatches narrow with
   `silent_grant_without_disclosure` or `read_only_missing_posture`.

4. **Override-route honesty.** Every non-`none` override route must
   reference both an override action id and an override disclosure id.
   Mismatches narrow with `override_route_undisclosed`.

5. **Support-export honesty.** Each surface's support-export
   projection must preserve `surface_kind`, `gate_decision`,
   `override_route`, `silent_execution_posture`, and `disclosure_id`,
   redact raw secrets, approval tickets, delegated credentials, and
   live authority handles, and (for credential-touching surfaces)
   declare a non-`local_only` posture. Dropping a field narrows with
   `support_export_fields_dropped`; raising raw material narrows with
   `support_export_redaction_unsafe`; a credential-touching surface
   shipping `local_only` narrows with `support_export_posture_unsafe`.

6. **Inspection precedes destructive grants.** A controlled inspection
   / repair hook table must be available before any destructive grant.
   The required classes are `inspect_trust_grant`, `review_grant_scope`,
   `compare_workspace_trust`, `rollback_grant`, `export`, and
   `repair`. A missing hook narrows with `inspection_hook_unavailable`.
   Worked example:
   [`missing_review_grant_hook_narrowed.json`](../../../fixtures/workspace/m4/trust_gating_lineage/missing_review_grant_hook_narrowed.json).

7. **Producer attribution is pinnable for replay.** Each record
   carries the producer ref, the schema version, the capture
   timestamp, and an integrity hash derived from the input surface
   identities so replay and support pipelines can pin the source
   before applying. Incomplete attribution narrows with
   `producer_attribution_incomplete`.

8. **Lineage and export stay honest.** Every record sets
   `raw_payload_excluded = true` and carries only opaque refs to the
   source workspace, corpus, and producer. An empty workspace or
   corpus ref narrows with `lineage_export_unsafe`.

9. **The record is replay-gated.** The replay gate re-projects each
   fixture and asserts it equals the checked-in `expected`, so the
   projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                  | Workspace trust posture | Surfaces observed                                              | Qualification           | Proves                                                                       |
| ---------------------------------------- | ----------------------- | -------------------------------------------------------------- | ----------------------- | ---------------------------------------------------------------------------- |
| `trusted_grant_stable`                   | `trusted`               | All five surfaces gated by `allow_after_explicit_grant`        | `stable`                | A trusted workspace still requires a disclosed grant per privileged surface  |
| `restricted_blocked_stable`              | `restricted`            | All five surfaces gated by `block_until_repair`                | `stable`                | A restricted workspace blocks every privileged surface until trust repair    |
| `pending_blocked_stable`                 | `pending_evaluation`    | All five surfaces gated by `block_pending_trust_decision`      | `stable`                | A pending workspace blocks every privileged surface until trust review       |
| `restricted_read_only_stable`            | `restricted`            | All five surfaces gated by `allow_read_only`                   | `stable`                | A restricted workspace can offer read-only inspect routes per surface        |
| `missing_review_grant_hook_narrowed`     | `trusted`               | All five surfaces gated by `allow_after_explicit_grant`        | `narrowed_below_stable` | Destructive grant with no grant-scope review hook narrows                    |

## How to verify

```sh
# Unit + replay gate for the trust-gating lineage projection.
cargo test -p aureline-workspace --lib trust_gating_lineage
cargo test -p aureline-workspace --test trust_gating_lineage_replay

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_trust_gating_lineage -- --lines \
  fixtures/workspace/m4/trust_gating_lineage/trusted_grant_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: surfaces that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public scope
is widened from this row.
