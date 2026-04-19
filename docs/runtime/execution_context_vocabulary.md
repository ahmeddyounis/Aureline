# Execution-context and workset / scope vocabulary

This document is the cross-surface companion to
[`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md).
The ADR freezes the decision; this document names the vocabulary
every non-resolver surface (terminal host, task runner, test runner,
debug adapter, notebook kernel controller, scaffolding engine, AI
tool-call plane, remote-agent attach service, Project Doctor probe
runner, `aureline env inspect`, `aureline doctor --explain`, support-
bundle exporter, replay artifact exporter, evidence-packet
exporter, mutation-journal renderer, profile import / export)
uses when it renders, logs, exports, or explains an execution.

If this document and the ADR disagree, the ADR wins and this
document must be updated in the same change.

## Artifacts this vocabulary points at

- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — boundary schema for the execution-context record, the
  execution-context provenance record, the standalone scope
  descriptor record, and the execution-context audit event record.
  Exports the frozen enums for `target_class`, `toolchain_class`,
  `activation_strategy`, `activator_class`, `activator_disposition`,
  `scope_class`, `lens_class`, `expansion_step_kind`,
  `partial_truth_label`, `authority_class`,
  `authority_envelope_tag`, `cache_disposition`,
  `degraded_field_reason`, and `audit_event_id`. The eventual
  execution-context crate's Rust types are the schema of record;
  this file is the cross-tool boundary.
- [`/artifacts/architecture/execution_context_tradeoff_rows.yaml`](../../artifacts/architecture/execution_context_tradeoff_rows.yaml)
  — machine-readable tradeoff register (ten axes, per-row reopen
  triggers) backing the ADR.
- [`/artifacts/runtime/execution_scope_matrix.yaml`](../../artifacts/runtime/execution_scope_matrix.yaml)
  — machine-readable matrix that binds each frozen `scope_class` to
  its minimum fields, its canonical authority class, its admissible
  narrowing / widening transitions, its admissible lenses, and its
  conformance tests.
- [`/fixtures/runtime/execution_context_examples/`](../../fixtures/runtime/execution_context_examples/)
  — short fixtures exercising the target-class enum, the
  toolchain-class enum, the scope-class enum, the authority
  envelope, the degraded-field taxonomy, and the provenance record
  shape.

## Vocabulary surfaces share

Every execution-aware surface renders and logs the fields below
using the exact names the schema exports. Aliases, cute labels, or
private renames on protected surfaces are forbidden.

- `execution_context_id` — monotonic id round-trips into every
  downstream event. Human surfaces may render a short alias but
  MUST be able to surface the id on demand.
- `invocation_subject` — `command_id`, `surface` (`task`, `test`,
  `debug`, `terminal`, `notebook_kernel`, `scaffolding`,
  `ai_tool_call`, `doctor_repair`, `import_probe`,
  `replay_probe`), `actor_class`, `workspace_id`, `profile_id`.
- `target_identity` — `target_class`, `canonical_target_id`,
  `requested_target_ref`, `materialised_instance_ref`,
  `mount_identity` (five-layer filesystem-identity record from
  ADR-0006), `route_dependency`, `reachability_state`,
  `capability_envelope_ref`.
- `toolchain_identity` — `toolchain_class`, `toolchain_id`,
  `resolved_version`, `executable_identity`, `activation_strategy`,
  `wrapper_provenance`, `extension_pack_refs`,
  `known_unsupported_gaps`, `degraded_fallback_flag`.
- `environment_capsule_ref` — `capsule_id`, `capsule_hash`,
  `resolved_schema_version`, `workspace_template_ref`,
  `prebuild_snapshot_ref`, `drift_state`,
  `compatibility_fingerprint`. Capsule bodies are content-addressed;
  this record quotes the reference.
- `workset_scope` — discriminated union over `current_root`,
  `named_workset`, `sparse_slice`, `full_workspace`,
  `policy_limited_view`, `review_workspace`, `companion_surface`.
  Every execution carries exactly one scope descriptor.
- `trust_state`, `identity_mode`, `policy_epoch` — copied from the
  workspace authority (ADR-0001, ADR-0008). Readers project; they
  do not assign.
- `activator_decisions` — ordered list of
  `activator_decision_record` entries. Applied, blocked, ignored,
  unsupported, and degraded activators all appear.
- `override_delta` — class labels for per-run overrides; raw
  values never cross this boundary.
- `cache_disposition` — `cold`, `warm`, `prebuild_reused`,
  `capsule_reused`, `rejected_drift`, `rejected_policy`,
  `rejected_trust`. Pairs with `cache_key_ref` and
  `invalidation_reason` where applicable.
- `provenance_record_ref` — reference to the
  `execution_context_provenance_record` that powers "why this
  execution context?" across every explainer surface.
- `authority_envelope` — ordered list of per-field authority tags
  from the ADR-0005 matrix.
- `degraded_fields` — ordered list of typed
  `degraded_field_record` entries. Empty means fully resolved;
  non-empty forces a visible honesty marker.

## Rules surfaces follow

1. **One execution-context record shape.** Every launch-capable or
   scope-narrowing surface reads from the execution-context
   resolver. No surface mints a private launch descriptor; no
   surface invents a private workset vocabulary.
2. **One scope per execution.** Ambient unscoped execution is
   forbidden on protected surfaces. Every execution declares
   exactly one `workset_scope`; lenses narrow presentation only and
   MUST NOT evade the scope's membership authority.
3. **Authority tags on every projected field.** Every field a
   surface renders carries an `authority_envelope` entry naming
   the canonical authority class (authoritative),
   `projected_from_<class>`, or `stale`. A field rendered without
   a tag is a conformance bug.
4. **Fail closed.** Target identity mismatches, toolchain identity
   mismatches, and unmet capability dependencies deny or visibly
   downgrade; silent best-effort execution is forbidden.
5. **Scopes narrow, they do not silently widen.** A sparse slice
   never silently grows into a named workset; a named workset
   never silently grows into a full workspace; a policy-limited
   view never silently exposes its hidden members. Widening
   requires an explicit user, migration, or approval action and is
   recorded in the mutation journal.
6. **Honest degradation.** Every unresolved, stale, or partially
   resolved field emits a `degraded_field_record` the consuming
   surface renders. A projecting surface MUST NOT re-render a
   degraded field as authoritative.
7. **One provenance record.** The same record powers UI
   explainers, `aureline env inspect`, `aureline doctor --explain`,
   support bundles, replay artifacts, and Project Doctor probes.
   Per-feature provenance formats are forbidden.
8. **Redaction on export.** Every surface that exports an
   execution-context record or a provenance record to a log, trace,
   support bundle, evidence packet, profile export, optional-sync
   payload, crash dump, mutation-journal entry, save manifest,
   replay / timeline capture, terminal transcript, or clipboard
   projection applies ADR-0007 redaction defaults. Raw environment
   bodies, raw command lines, and raw secret values require an
   explicit `broadened_capture` opt-in recorded in the provenance
   record.
9. **Mutation journal carries execution references.** Every
   mutation-journal entry that references an execution names
   `execution_context_id`, `target_class`, `toolchain_id`,
   `workset_scope.scope_class`, and `provenance_record_id`. The
   journal MUST NOT embed raw env / command / secret material.
10. **Audit every observable action.** Every resolver action a
    user, administrator, support engineer, or governance reviewer
    could ask about emits one of the frozen audit events. Audit
    events never carry raw env / command / secret material.

## Where related decisions live

- Identity modes and workspace trust:
  [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md).
- RPC transport execution-context records ride across:
  [`docs/adr/0004-rpc-transport-and-schema-toolchain.md`](../adr/0004-rpc-transport-and-schema-toolchain.md).
- Subscription envelope and authority-class matrix execution-
  context fields project into:
  [`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- Filesystem-identity layers target mounts re-export:
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- Secret broker every projected credential resolves against:
  [`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md).
- Settings resolver whose `resolved_scope` taxonomy this ADR
  projects:
  [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md).
- Decision register row tracking `D-0015`:
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## Change management

- Adding a new `target_class`, `toolchain_class`,
  `activation_strategy`, `activator_class`, `scope_class`,
  `lens_class`, `authority_envelope_tag`, `audit_event_id`, or
  `degraded_field_reason` is additive-minor: bump
  `execution_context_schema_version`, add a row in the relevant
  matrix, and extend the schema.
- Repurposing any existing value (for example, reusing an existing
  `degraded_field_reason` for a different fail path) is breaking
  and requires a new decision row.
- Renaming a `toolchain_id`, `activator_id`, or `workset_id` is
  done through the registry's alias mechanism (mirrors ADR-0008's
  alias discipline); it is never done by mutating the existing
  canonical id.
