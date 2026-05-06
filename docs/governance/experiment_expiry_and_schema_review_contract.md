# Experiment expiry, graduation/demotion, and schema-governance review contract

This document publishes the **normative** contract that keeps
experiments (feature flags, Labs rows, rollouts, and other lifecycle
gates) and schema-governed surfaces reviewable across import, sync,
downgrade, restore, and support-export flows.

The primary goals are:

- expiry and stale-state never disappear silently;
- graduation, demotion, retirement, and forced-disable actions produce
  explicit review packets instead of ad-hoc local edits; and
- schema mismatch and unknown-field handling names what was preserved,
  dropped, blocked, or deferred with copy-safe output.

Machine-readable companions:

- [`/schemas/governance/experiment_review_row.schema.json`](../../schemas/governance/experiment_review_row.schema.json)
  — boundary schema for an `experiment_review_row` record that release,
  docs, support, settings, and claim publication can reuse without
  local extensions.
- [`/artifacts/governance/experiment_graduation_matrix.yaml`](../../artifacts/governance/experiment_graduation_matrix.yaml)
  — matrix of lifecycle actions (graduate, demote, retire, forced
  disable) plus the mismatch-blocking and stale-state preservation
  rules those actions must honor.
- [`/fixtures/governance/experiment_edge_cases/`](../../fixtures/governance/experiment_edge_cases/)
  — worked edge cases for import, sync conflict, downgrade, restore,
  and support export.

Related upstream contracts (this document composes over them and does
not replace them):

- [`/docs/governance/feature_flag_policy.md`](./feature_flag_policy.md)
  and [`/artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml)
  — per-row governance and the canonical register of control rows.
- [`/docs/governance/policy_flag_schema_stack.md`](./policy_flag_schema_stack.md)
  and [`/artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml)
  — schema-family ownership, unknown-field policy, and contract
  selection for machine-readable artifacts.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  §7.2.18 — experiments, Labs, and capability-lifecycle architecture.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  §18.25 — required UX cues for flags, policy decisions, and schema
  validation.

## Terms and invariants

The terms below are used consistently in this repository:

- **Control row**: a governed experiment, feature flag, benchmark mode,
  or rollout row registered in `experiments_register.yaml`.
- **Review row**: a reusable, projection-friendly record that captures
  the operator- and user-visible contract of a control row (owner,
  lifecycle, expiry, kill-switch source, consequences, migration note,
  and schema impact). Review rows conform to
  `experiment_review_row.schema.json`.
- **Schema-governed surface**: any artifact family whose shape is
  governed by a JSON schema family row (settings, profiles, workspace
  manifests, support bundles, exports, migration packets, etc.).

Non-negotiable invariants:

1. **No silent disappearance.** Expired, unsupported, or policy-disabled
   experiments do not vanish into generic defaults. They downgrade
   visibly and preserve an exportable reason.
2. **No silent rewrite on schema mismatch.** Import/sync/downgrade flows
   do not silently rewrite user-owned durable truth on version mismatch.
   They produce explicit mismatch cues that name preserve/drop/block/defer.
3. **Copy-safe output.** Any surfaced id, field path, version, or reason
   is safe to paste into an issue, support ticket, or team chat without
   including secret-bearing payload values.

## Review row contract

Every control row that can affect a schema-governed surface MUST be
reviewable as one `experiment_review_row` record:

- **Owner and scope:** `owner_dri`, optional owning scope metadata, and
  `cohort_or_ring`.
- **Public label and lifecycle:** `public_label` and `lifecycle_state`
  use the controlled vocabulary; adjacent surfaces must not invent
  marketing synonyms.
- **Expiry or review date:** at least one of `review_by` or `expires_on`
  is set (dates are explicit and inspectable).
- **Kill-switch source:** the source layer that can force-disable (or
  narrow) the row is explicit and refers to a concrete artifact path or
  authority record.
- **Consequence classes:** the row declares whether enabling it can
  change telemetry, route selection, cost posture, write-scope, or
  retention posture. If it can, the review row carries a short summary
  per consequence class.
- **Migration note:** a short, reviewer-readable note exists even when
  the migration is “none” (so review surfaces never need to special-case
  missing fields).
- **Schema impact:** the row lists schema-governed surfaces it can
  affect, the schema reference, and the exact field paths that would be
  added/removed/repurposed under the experiment.

The intent is projection reuse: release notes, docs badges, settings
explainers, support exports, and claim-publication automation all
consume the same review row instead of re-authoring parallel summaries.

## Lifecycle actions and packets

Promotion and rollback are governance events, not just code changes.
Graduation, demotion, retirement, and forced-disable actions follow the
packet requirements in `experiment_graduation_matrix.yaml`.

At minimum, a lifecycle action packet:

- updates the canonical register row (`experiments_register.yaml`);
- updates (or adds) the corresponding `experiment_review_row`;
- cites the schema impacts and mismatch behavior if any schema-governed
  surface is affected; and
- updates or adds an edge-case fixture when the action introduces a new
  stale/expiry/mismatch behavior.

When schema or policy mismatch is detected, the action MUST either:

- **block enable/apply** (fail closed), or
- **narrow scope** (reduce cohort/ring or reduce write/retention scope),
  while preserving the mismatch cues for export/support.

## Schema mismatch cues (copy-safe)

Whenever an import/sync/downgrade/restore path encounters a schema
mismatch that could change durable user-owned truth, it emits one or
more mismatch cues. Each cue names:

- **artifact type** (which surface is affected),
- **schema version** (observed vs supported),
- **exact field path** (no values; only paths),
- **severity** (`info`, `warning`, or `error`),
- **preserve/drop behavior** (what happened or will happen), and
- **docs link** (where to learn the contract and remediation path).

The cue format is intentionally copy-safe: it is suitable for CLI/stdout
structured output, support exports, and issue templates.

## Stale/expired experiment handling across flows

Expired or unsupported control rows must remain visible through:

- **Profile import:** imported artifacts keep a dependency marker (or
  tombstone) for any referenced control row; apply is blocked or narrowed
  when the missing capability would widen authority or change durable
  shape.
- **Sync conflict review:** conflicts record `missing-capability` and
  `stale-remote` cases explicitly; “keep local” and “keep remote” never
  erase the downgrade reason.
- **Downgrade/open:** opening newer durable state on an older build keeps
  stale experiment dependencies inspectable and exportable; the product
  never rewrites them away during “repair” unless the user accepts an
  explicit preserve/drop plan.
- **Restore:** restoring a checkpoint preserves the same dependency and
  mismatch cues the checkpoint captured; restore does not erase expired
  control-row provenance.
- **Support export:** support bundles include the stale/expired control
  rows and their reasons as copy-safe records rather than collapsing them
  into generic defaults.

Worked examples live under `fixtures/governance/experiment_edge_cases/`.

