# Execution-context inspector, snapshot, and provenance-diff packet

This packet freezes one shared inspectable execution-context model before
the task launch, terminal session seed, and debug-prep seed paths
diverge. It names the snapshot object every launch-capable surface
emits, the structured diff later support/export and explainer flows
consume, and the compact inspector view a CLI or dev-only panel
renders. It exists so the same context can be compared across runs or
across surfaces using one reviewable object model instead of
per-surface prose.

If this packet, the
[`context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
boundary, the seed examples in
[`/artifacts/execution/context_examples/`](../../artifacts/execution/context_examples/),
and the diff cases in
[`/fixtures/execution/context_diff_cases/`](../../fixtures/execution/context_diff_cases/)
disagree, the machine-readable schema and the frozen
execution-context vocabulary in
[`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
win for tooling and this packet must update in the same change.

Companion artifacts:

- [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
  — boundary schema for the `context_snapshot_record`, the
  `context_snapshot_diff_record`, and the
  `context_inspector_view_record`. Re-exports the frozen vocabulary
  from the runtime execution-context schema without minting new
  authority, target, toolchain, or scope classes.
- [`/artifacts/execution/context_examples/`](../../artifacts/execution/context_examples/)
  — one seed snapshot for each of the three surfaces this packet
  binds together (`task_launch`, `terminal_session_seed`,
  `debug_prep_seed`) plus an inspector-view projection.
- [`/fixtures/execution/context_diff_cases/`](../../fixtures/execution/context_diff_cases/)
  — reviewer-facing diff cases covering exact match, environment
  drift, wrong target, policy-limited context, and
  degraded / unknown / redaction-limited fields.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  — canonical cross-surface execution-context vocabulary the
  snapshot quotes by field name.
- [`/artifacts/runtime/execution_scope_matrix.yaml`](../../artifacts/runtime/execution_scope_matrix.yaml)
  — canonical workset / scope matrix; the snapshot's `scope_class`
  is a pass-through of the row id.
- [`/docs/verification/target_and_host_boundary_packet.md`](../verification/target_and_host_boundary_packet.md)
  — target-truth record shape that sits upstream of this inspector;
  the wrong-target diff case is the complement on the
  execution-context side.
- [`/docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md)
  — Project Doctor finding-record contract; Doctor consumes snapshots
  and diffs by citing `execution_context_id` and `diff_id`.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — diagnostics, supportability, and export-redaction posture;
  "why this execution context" and "why this target / toolchain"
  explainers treated as in-product contracts rather than afterthoughts.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — execution-context provenance, authority-class matrix, subscription
  envelope, and redaction-default definitions this packet projects.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — execution-context resolver, target identity, toolchain identity,
  environment capsule, activator decision, and override-delta shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — "why this toolchain?" / "why this target?" inspector surface
  copy rules the dev-only inspector view mirrors.
- `.t2/docs/Aureline_Milestones_Document.md`
  — shared execution context named as a release-blocking posture
  during the foundations phase.

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: execution_packet
packet_id: execution.context_inspector.seed
evidence_id: evidence.execution.context_inspector.packet
title: Execution-context inspector, snapshot, and provenance-diff packet
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - TOOL-CTX-002
    - TOOL-ENV-006
    - TOOL-EXEC-007
    - TOOL-INFRA-002
    - SEC-AUTHZ-011
    - SEC-CRED-009
    - SEC-TRUST-001
    - ARCH-COMP-005
    - ARCH-STATE-012
    - GOV-EVID-901
  claim_row_refs:
    - packet_row:context_inspector.snapshot_contract
    - packet_row:context_inspector.surface_parity
    - packet_row:context_inspector.diff_contract
    - packet_row:context_inspector.redaction_honesty
    - packet_row:context_inspector.degraded_and_unknown_honesty
    - packet_row:context_inspector.policy_limited_view_projection
    - packet_row:context_inspector.inspector_view_projection
    - packet_row:context_inspector.seed_corpus
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: context_inspector_seed@1
  trigger_revision: context_inspector_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen execution-context, authority-class,
    secret-broker, and redaction-default vocabularies. No live
    resolver, remote-agent broker, or Project Doctor surface is wired
    to this packet yet. Claims are structural: every snapshot,
    inspector view, and diff in the artifact set reuses the existing
    frozen tokens rather than minting new per-surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.execution.context_snapshot_schema
    - evidence.execution.context_examples_seed
    - evidence.execution.context_diff_cases_seed
    - evidence.runtime.execution_context_schema
    - evidence.runtime.execution_scope_matrix
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/execution/context_diff_cases/
    - fixtures/runtime/execution_context_examples/
    - artifacts/execution/context_examples/
  archetype_refs: []
  source_anchor_refs:
    - schemas/execution/context_snapshot.schema.json
    - schemas/runtime/execution_context.schema.json
    - docs/runtime/execution_context_vocabulary.md
    - artifacts/runtime/execution_scope_matrix.yaml
    - artifacts/architecture/execution_context_tradeoff_rows.yaml
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one `context_snapshot_record` shape every launch-capable surface
  (`task_launch`, `terminal_session_seed`, `debug_prep_seed`,
  `notebook_kernel_seed`, `ai_tool_call_seed`, `doctor_explain`,
  `env_inspect_cli`, `replay_probe`, `import_probe`,
  `support_export`) emits when it seeds, inspects, or exports a
  context;
- one `context_snapshot_diff_record` that compares two snapshots
  across runs or across surfaces, naming every changed layer and
  every preserved layer in a structured way later support-export,
  doctor-explain, and evidence-bundle flows can reuse;
- one `context_inspector_view_record` projection a CLI or dev-only
  inspector panel renders over a snapshot without minting its own
  vocabulary;
- one redaction-aware secret posture summary that names class labels
  and counts only — raw env bodies, raw command lines, raw paths,
  and raw secret values never cross this boundary;
- one seed corpus of snapshots and diffs covering exact match,
  environment drift, wrong target, policy-limited context, and
  degraded / unknown / redaction-limited fields.

It does not claim a live resolver, a live inspector CLI, or a live
diff exporter is wired up. It claims only that one inspectable
execution-context model exists in one reviewable form and reuses the
frozen runtime vocabularies already landed in this repository.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:context_inspector.snapshot_contract` | `TOOL-CTX-002`, `TOOL-ENV-006`, `ARCH-STATE-012` | `seed_only` | `internal` | `evidence.execution.context_snapshot_schema` | Freezes one machine-readable snapshot record every launch-capable surface emits. |
| `packet_row:context_inspector.surface_parity` | `TOOL-CTX-002`, `TOOL-EXEC-007` | `seed_only` | `internal` | `evidence.execution.context_examples_seed` | Same snapshot schema is emitted from task-launch, terminal-seed, and debug-prep seed fixtures; private per-surface launch descriptors are forbidden. |
| `packet_row:context_inspector.diff_contract` | `TOOL-CTX-002`, `TOOL-INFRA-002`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.execution.context_diff_cases_seed` | Freezes one machine-readable diff record with preserved / changed / degraded / unknown / redaction-limited layer status tokens. |
| `packet_row:context_inspector.redaction_honesty` | `SEC-CRED-009`, `SEC-TRUST-001`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.execution.context_snapshot_schema` | Raw env bodies, raw command lines, and raw secret values require an explicit `broadened_capture` opt-in with an `approval_ticket_ref`; the default redaction class is `metadata_and_hashes_only`. |
| `packet_row:context_inspector.degraded_and_unknown_honesty` | `TOOL-CTX-002`, `ARCH-COMP-005` | `seed_only` | `internal` | `evidence.execution.context_diff_cases_seed` | `degraded_on_<side>`, `unknown_on_<side>`, and `redaction_limited` statuses let the diff express partial resolution without silently claiming parity. |
| `packet_row:context_inspector.policy_limited_view_projection` | `SEC-AUTHZ-011`, `ARCH-STATE-012` | `seed_only` | `internal` | `evidence.execution.context_diff_cases_seed` | Policy-limited views record `hidden_member_count` and the narrowing authority; the exact hidden list never projects outside the policy-admin surface. |
| `packet_row:context_inspector.inspector_view_projection` | `TOOL-CTX-002`, `TOOL-ENV-006` | `seed_only` | `internal` | `evidence.execution.context_examples_seed` | One compact inspector-view record shape covers CLI / dev-only panels without minting parallel labels. |
| `packet_row:context_inspector.seed_corpus` | `TOOL-CTX-002`, `TOOL-EXEC-007`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.execution.context_diff_cases_seed`, `evidence.execution.context_examples_seed` | One stable case-id set now covers the required exact-match, drift, wrong-target, policy-limited, and degraded/unknown scenarios. |

## What this seed freezes

- One `context_snapshot_record` shape every launch-capable or
  inspector surface reuses.
- One `context_snapshot_diff_record` shape that compares two
  snapshots with a closed `diff_layer` set, a closed `diff_status`
  set, and an honest redaction-limited story.
- One `context_inspector_view_record` shape every CLI / dev-only
  inspector uses, bound to a closed `inspector_view_field` set.
- One redaction-aware secret-posture summary that names class
  labels and counts only.
- One seed corpus the support-export, release, and docs surfaces can
  cite by case id rather than re-deriving prose.

## Snapshot record

Every snapshot carries these required fields (see
[`context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
for the authoritative boundary):

- `record_kind` (`context_snapshot_record`)
- `context_snapshot_schema_version`
- `snapshot_id`
- `captured_at`
- `source_surface` — one of
  `task_launch`, `terminal_session_seed`, `debug_prep_seed`,
  `notebook_kernel_seed`, `ai_tool_call_seed`, `doctor_explain`,
  `env_inspect_cli`, `replay_probe`, `import_probe`, `support_export`
- `execution_context_id`
- `execution_context_record_ref` — points at the canonical
  execution-context record
- `provenance_record_ref` — points at the canonical
  execution-context provenance record
- `redaction_class` — default `metadata_and_hashes_only`
- `snapshot_summary` — flat key-to-value summary of the underlying
  execution-context record, carrying only class labels, frozen
  tokens, opaque ids, hashes, and counts
- `secret_posture_summary` — ADR-0007 secret-class labels and counts
  only
- `degraded_field_refs` — pass-through of the execution-context
  record's degraded-field list

Rule: a snapshot whose `redaction_class` is `broadened_capture` MUST
carry an `approval_ticket_ref`; a snapshot whose
`secret_posture_summary.raw_secret_values_present` is `true` MUST
carry an `approval_ticket_ref`. Silent inclusion of raw env bodies,
raw command lines, raw paths, or raw secret values is non-conforming.

Rule: snapshots from different source surfaces for the same
execution MUST share the same `execution_context_id`. Per-surface
private ids are forbidden.

Rule: a snapshot's `snapshot_summary` MUST NOT invent a new
`target_class`, `toolchain_class`, `scope_class`, `lens_class`,
`activation_strategy`, `cache_disposition`, or
`authority_envelope_tag`. The snapshot is a projection of the
canonical execution-context record, not a new vocabulary.

## Diff record

Every diff carries these required fields:

- `record_kind` (`context_snapshot_diff_record`)
- `diff_id`, `captured_at`
- `snapshot_a_ref`, `snapshot_b_ref`
- `snapshot_a_source_surface`, `snapshot_b_source_surface`
- `redaction_class`
- `layer_entries` — one entry per diff layer
- `preserved_layer_count`, `changed_layer_count`
- `redaction_limited_layer_count`
- `summary_headline`

### Diff layers

The closed layer set mirrors the frozen execution-context record's
top-level fields plus the secret-posture and authority-envelope
projections the snapshot exports:

- `invocation_subject`
- `target_identity`
- `toolchain_identity`
- `environment_capsule_ref`
- `workset_scope`
- `trust_state`
- `identity_mode`
- `policy_epoch`
- `activator_decisions`
- `override_delta`
- `cache_disposition`
- `secret_posture`
- `authority_envelope`
- `degraded_fields`

### Diff status vocabulary

| Status | When it applies |
|---|---|
| `preserved` | Both snapshots carry the same token / hash / count for the layer. |
| `changed` | A visible token / hash / count difference between the snapshots. |
| `added_on_b` | The layer is absent on A and present on B. |
| `removed_on_b` | The layer is present on A and absent on B. |
| `degraded_on_a` | The layer is unresolved or partially resolved on A, named by a frozen `degraded_field_reason`. |
| `degraded_on_b` | Same, on side B. |
| `unknown_on_a` | The resolver could not determine the layer's value on A. |
| `unknown_on_b` | Same, on side B. |
| `redaction_limited` | A fuller comparison would require `broadened_capture` and an approval ticket; the diff names a typed `redaction_limited_reason`. |

Rule: `redaction_limited` MUST NOT be used to hide a visible token
change. If two class labels differ, the status is `changed` and the
labels project; `redaction_limited` records only the part of the
comparison that would require broader capture.

Rule: a layer whose status is `degraded_on_<side>` MUST carry a
`degraded_reason` drawn from the frozen `degraded_field_reason`
vocabulary. A layer whose status is `redaction_limited` MUST carry
a `redaction_limited_reason`.

## Inspector view

A CLI or dev-only panel renders snapshots using the
`context_inspector_view_record`, which projects over the snapshot's
summary plus its secret-posture and degraded-field lists. The view's
row set is the closed `inspector_view_field` list in the schema. No
field outside the list projects; private per-inspector labels are
non-conforming.

Rule: every inspector-view row carries an
`authority_envelope_tag` drawn from the frozen ADR-0005 vocabulary.
A row without a tag is a conformance bug.

Rule: the inspector view MAY NOT render a field as authoritative when
the underlying snapshot carries a matching `degraded_field_ref` for
the field. Use the `degraded_reason` column instead.

## Redaction and secret posture

- Default `redaction_class` is `metadata_and_hashes_only`. Raw env
  bodies, raw command lines, raw paths, and raw secret values do not
  cross the boundary under this class.
- `broadened_capture` requires an `approval_ticket_ref` on the
  enclosing snapshot / diff / view. Silent inclusion of raw bytes is
  non-conforming.
- `secret_posture_summary` carries the ADR-0007 secret class labels
  and counts — never raw secret values. `raw_secret_values_present`
  is always `false` unless `redaction_class` is `broadened_capture`.
- Credential alias handles project as counts. The aliases themselves
  are opaque ids; the underlying key material is not part of this
  packet.

## Surface parity

Three surfaces bind to the snapshot shape in this seed:

| Source surface | Seed snapshot | Underlying execution-context fixture |
|---|---|---|
| `task_launch` | [`task_launch_snapshot.json`](../../artifacts/execution/context_examples/task_launch_snapshot.json) | [`local_task_launch.json`](../../fixtures/runtime/execution_context_examples/local_task_launch.json) |
| `terminal_session_seed` | [`terminal_session_seed_snapshot.json`](../../artifacts/execution/context_examples/terminal_session_seed_snapshot.json) | [`remote_ssh_attach.json`](../../fixtures/runtime/execution_context_examples/remote_ssh_attach.json) |
| `debug_prep_seed` | [`debug_prep_seed_snapshot.json`](../../artifacts/execution/context_examples/debug_prep_seed_snapshot.json) | [`devcontainer_launch.json`](../../fixtures/runtime/execution_context_examples/devcontainer_launch.json) |

An inspector-view projection over the task-launch snapshot lives in
[`inspector_view_task_launch.json`](../../artifacts/execution/context_examples/inspector_view_task_launch.json).

## Seed diff cases

| Case id | Status mix | Coverage |
|---|---|---|
| `exact_match` | 14 preserved | Two runs of the same task launch match on every frozen layer. |
| `environment_drift` | 12 preserved, 2 changed | Capsule hash and cache disposition change; target and toolchain identity preserved. |
| `wrong_target` | 4 preserved, 9 changed, 1 degraded_on_b | Target moved from `local_host` to `ssh_remote`; route promotes to tunneled; reachability degraded on side B. |
| `policy_limited_context` | 9 preserved, 5 changed | Admin policy narrowed the scope to `policy_limited_view` with `hidden_member_count = 3`; exact hidden list not projected. |
| `degraded_unknown_fields` | 7 preserved, 3 changed, 3 degraded_on_b, 1 redaction_limited | Side B ran under restricted trust: activator blocked, toolchain fell back, override_delta comparison capped by redaction. |

Each case is one `context_snapshot_diff_record` in
[`/fixtures/execution/context_diff_cases/`](../../fixtures/execution/context_diff_cases/)
with a stable `case_id` registered in
[`manifest.yaml`](../../fixtures/execution/context_diff_cases/manifest.yaml).

## Rules surfaces follow

1. **One snapshot shape.** Every launch-capable or inspector surface
   emits a `context_snapshot_record` and MAY NOT mint a private
   launch descriptor or private inspector object.
2. **Shared schema, shared ids.** Task-launch, terminal-seed, and
   debug-prep-seed snapshots of the same execution share the same
   `execution_context_id`. Surface-local ids are forbidden.
3. **One diff shape.** Comparing two executions uses the
   `context_snapshot_diff_record`. Private per-surface diff prose is
   non-conforming.
4. **Redaction is explicit.** `broadened_capture` needs an
   `approval_ticket_ref`; silent inclusion of raw env, raw argv, or
   raw secret values is non-conforming.
5. **Honest degradation.** Unresolved or partially resolved layers
   project a `degraded_on_<side>` or `unknown_on_<side>` status with
   a frozen reason token. A diff MUST NOT silently re-assert parity
   for a field whose value was not observable.
6. **Scope narrowing is structured.** A `policy_limited_view` diff
   entry records `hidden_member_count` and the narrowing authority;
   the exact hidden list never leaves the policy-admin surface.
7. **Inspector rows are authority-tagged.** Every row in a
   `context_inspector_view_record` carries an
   `authority_envelope_tag`; a missing tag is a conformance bug.
8. **No milestone leakage.** Snapshots, diffs, and inspector views
   MUST NOT carry milestone or task identifiers; they carry the
   stable execution-context vocabulary only.

## Change management

- Adding a new `snapshot_source_surface`, `diff_layer`,
  `diff_status`, or `inspector_view_field` is additive-minor: bump
  `context_snapshot_schema_version`, extend the schema, and add a
  row in the relevant table here.
- Repurposing an existing value (for example, reusing
  `redaction_limited` for a case that is actually `changed`) is
  breaking and requires a new decision row.
- Removing a case from the seed corpus is breaking; adding a case
  that exercises a new status combination is additive.
