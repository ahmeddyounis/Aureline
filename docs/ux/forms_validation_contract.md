# Form validation, probe, and staged-review contract

This document freezes the shared UX contract for high-impact forms that
validate, probe, stage, review, and then apply changes. It exists so
settings, connection setup, package actions, policy edits, repair
actions, network / transport forms, request / runtime forms, and any
future flow that validates external targets or secrets use one review
model instead of local status badges and one-off "test connection"
rules.

The contract is normative. Where this document disagrees with the
source UI / UX, architecture, or domain contract it cites, the source
wins and this document plus its schemas and fixtures update in the same
change. Where this document disagrees with a downstream surface's
private form wording, this document wins and the surface is
non-conforming.

Companion artifacts:

- [`/schemas/ux/form_probe_state.schema.json`](../../schemas/ux/form_probe_state.schema.json)
  - boundary schema for one validation or probe result. It preserves
  source attribution, freshness, target identity, scope, safety class,
  result tier, stale / skipped handling, and apply-gate semantics.
- [`/schemas/ux/staged_review_state.schema.json`](../../schemas/ux/staged_review_state.schema.json)
  - boundary schema for one form review sheet or equivalent CLI /
  headless review packet. It binds live value, staged value, validation
  rollup, field rows, policy locks, apply actions, and support handoff.
- [`/fixtures/ux/form_validation_cases/`](../../fixtures/ux/form_validation_cases/)
  - worked YAML records covering settings, connection setup, package
  actions, policy edits, repair actions, transport forms, and request /
  runtime forms.

This contract composes with, and does not replace:

- [`/docs/settings/settings_vocabulary.md`](../settings/settings_vocabulary.md)
  for effective setting, write-intent, preview, checkpoint, and lock
  vocabulary.
- [`/docs/admin/policy_explainability_contract.md`](../admin/policy_explainability_contract.md)
  for policy source, owner, freshness, validation, and lock
  explanation.
- [`/docs/package/package_action_contract.md`](../package/package_action_contract.md)
  for package review packets, script-risk, lockfile-impact, rollback,
  and mirror / offline posture.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  for repair preview, checkpoint, reversal, and forbidden-action
  semantics.
- [`/docs/network/transport_governance_seed.md`](../network/transport_governance_seed.md)
  for proxy, trust-store, mirror route, offline, and deny-all
  transport posture.
- [`/docs/api/request_workspace_contract.md`](../api/request_workspace_contract.md)
  for request, environment, assertion, stale schema, replay, and raw
  secret handling.
- [`/docs/data/database_tooling_contract.md`](../data/database_tooling_contract.md)
  for connection profiles, brokered credentials, statement safety, and
  result-grid export truth.
- [`/docs/ux/live_update_review_contract.md`](./live_update_review_contract.md)
  for stale and live-review honesty when a form depends on moving
  external state.

Normative source sections projected here include the form / validation
templates in `.t2/docs/Aureline_UI_UX_Spec_Document.md`, the repair and
probe architecture in `.t2/docs/Aureline_Technical_Architecture_Document.md`,
the data / request / live-action matrices in
`.t2/docs/Aureline_Technical_Design_Document.md`, the schema-governance
guardrails in `.t2/docs/Aureline_PRD.md`, and the component-state
vocabulary in `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`.

## Scope

This contract applies to any form or parameter surface where an apply
action can:

- mutate user, workspace, managed, remote, package, policy, route,
  request, runtime, or repair state;
- validate against an external target, provider, secret handle,
  credential broker, policy bundle, package registry, database,
  runtime, route, or remote endpoint;
- stage a value before commit; or
- produce support, admin, audit, issue-handoff, or review evidence.

Low-risk live controls such as local editor display toggles MAY apply
immediately without a staged review record. The moment a control crosses
external state, secret-bearing state, policy-owned state, broad
workspace state, or durable repair / package / request state, it enters
this contract.

## Two Records

Every conforming implementation emits the records below when the state
exists. A surface MAY render them inline, in a sheet, in CLI JSON, or in
a headless summary, but it may not rename the state vocabulary.

### `form_probe_state_record`

One probe-state record describes one validation or probe result. It
answers:

- who or what produced the result;
- which target and scope were checked;
- whether the result is current, stale, pending, failed, skipped,
  unsupported, or policy-blocked;
- whether the probe was local, external, dry-run, declared mutating, or
  forbidden from running as a probe;
- whether apply is allowed, blocked, or allowed only with stale /
  skipped acknowledgement; and
- which evidence refs survive into support exports and issue handoff.

The record is intentionally one row, not a free-text "test connection"
receipt. If a connection form checks DNS, TLS, auth, and schema
freshness, each check gets its own probe row or a domain packet that
links to equivalent rows.

### `staged_review_state_record`

One staged-review record describes the review state for a whole form.
It binds:

- the live basis and staged basis;
- field rows, source classes, redaction classes, and lock states;
- synchronous validation state;
- asynchronous probe state;
- stale / skipped handling;
- mutation-blocking tier;
- available apply actions; and
- support / issue-handoff exportability.

The record backs visible review sheets, settings apply bars, package
review panels, repair previews, connection setup review, request replay
review, CLI `--json` output, and support-bundle previews. A surface may
use a compact view only if the full record remains reachable.

## Validation Classes

Every validation result names exactly one `validation_class`.

| Class | Meaning | Default consequence |
|---|---|---|
| `local_syntax` | Client-local parse, shape, regex, path shape, or literal format check. | Inline error or warning. |
| `schema` | JSON Schema, request schema, settings definition, package manifest, policy bundle, or domain-schema check. | Inline linked error; may block apply. |
| `local_capability` | Checks local capability availability without external I/O. | Warning or block depending on required capability. |
| `secret_resolution` | Validates that a secret handle or delegated credential can be referenced without revealing the secret. | Blocks secret-bearing mutation when missing or raw. |
| `external_probe` | Network, provider, runtime, registry, database, route, or remote endpoint probe. | Pending, current, stale, failed, skipped, or policy-blocked. |
| `remote_auth` | Auth, session, broker, delegated identity, or reauth state. | Reauth / open-details path; blocks when required. |
| `policy` | Policy source, lock, entitlement, route, egress, or managed-admin constraint. | Policy explanation; policy blocks deny apply. |
| `dry_run` | Non-mutating preview, plan, resolver, package lock simulation, request assertion, or repair preview. | Preview / review step before apply. |
| `dependency_graph` | Package, import, build target, or workspace graph impact check. | Review impact; may block if stale or failed. |
| `target_discovery` | Target identity, runtime origin, route, database, package registry, or execution target discovery. | Requires visible target and freshness before mutation. |

Synchronous validation MUST run without external side effects. It may
read local staged values, local schema definitions, policy records that
are already present, effective settings, and broker handles by ref. It
MUST NOT contact a provider, refresh a policy bundle, run repo-owned
hooks, open a network connection, reveal a secret, or mutate target
state.

Asynchronous probes may observe external state only when their
`probe_safety_class` declares that posture. They MUST be cancelable,
timeout-bounded, attributable, and exportable as metadata. They MUST NOT
smuggle mutation into a validation path.

## Probe Attribution

Every external, remote, auth, transport, package, database, repair, or
target-discovery probe MUST preserve:

- `probe_source.source_class`;
- `probe_source.source_ref`;
- `probe_source.probe_version`;
- `target.target_class`;
- `target.target_ref`;
- `scope.scope_class`;
- `scope.scope_ref`;
- `probe_safety_class`;
- `freshness.freshness_class`;
- `freshness.captured_at` when a probe ran;
- `freshness.stale_after` when the result can age out;
- `apply_gate.apply_action_may_proceed`;
- `apply_gate.apply_action_may_proceed_with_stale`;
- `apply_gate.apply_action_may_proceed_when_skipped`; and
- `attribution.evidence_refs` or an explicit empty list.

Support exports and issue handoff MUST keep these fields even when
payloads, raw endpoints, raw URLs, raw request bodies, raw policy
bundles, raw certificate material, raw package registry addresses, raw
database coordinates, and raw secret values are omitted. The user and
support engineer should still be able to see which checker ran, what
class of target it checked, how fresh the result was, and why apply was
or was not allowed.

## Freshness And Staleness

Validation freshness is an apply-gate input, not decoration.

| Freshness | Meaning | Apply default |
|---|---|---|
| `current` | Result is inside its declared freshness window and still matches the staged basis. | Allowed if result tier allows. |
| `warm_within_grace` | Result is aged but still inside an explicit grace window. | Allowed only if the apply gate says stale may proceed. |
| `stale_within_grace` | Result aged past current but may still support a reviewer decision with acknowledgement. | Requires stale disclosure and explicit gate. |
| `stale_beyond_grace` | Result may no longer justify mutation. | Blocks mutation until revalidated unless the flow is observe-only. |
| `pending` | Probe is running or queued. | Blocks required mutation-class apply. |
| `skipped` | Probe did not run because of user choice, policy, offline posture, or unsupported target. | Blocks unless skipped apply is explicitly admitted. |
| `unknown` | The producer cannot establish freshness. | Treat as stale beyond grace for mutation-class actions. |

When a user edits any field that affects a probe target, target scope,
credential handle, policy source, or dry-run basis, dependent probe rows
MUST move to stale or pending before apply appears available. A surface
that leaves a previous green result as if it were current after the
target changed is non-conforming.

## Live Versus Staged Apply

Every review record declares an `apply_timing_class`.

| Timing | Allowed use | Rule |
|---|---|---|
| `immediate_live_apply` | Low-risk local controls with no external, broad, secret, policy, or repair side effect. | No staged sheet required. |
| `staged_apply_required` | Settings, connection profiles, policy edits, or request environments where fields can be reviewed before mutation. | Apply reads staged values, not live controls. |
| `preview_first_apply_required` | Package, repair, migration, publish, route, database write, or external mutation flows. | A preview / dry-run / checkpoint record must exist before apply. |
| `dry_run_then_apply` | Infrastructure, package, repair, request replay, or runtime actions where a plan validates target effects. | Dry run stays non-mutating and attributable. |
| `policy_locked_no_apply` | Policy or managed-admin state forbids mutation. | Apply is unavailable; explanation action remains available. |
| `observe_only_no_apply` | Support, audit, export, or read-only inspection. | No mutating apply action is rendered. |

Live fields may update a local draft. They do not mutate the target
until the staged review has satisfied the gate. If live external state
changes while a staged review is open, the review stays pinned to its
baseline and the changed dependency is surfaced as stale, pending, or
blocked before apply.

## Mutation-Blocking Tiers

Every staged review has one `mutation_blocking_tier`.

| Tier | Meaning | Apply behavior |
|---|---|---|
| `none` | No validation issue gates apply. | Apply may be enabled. |
| `advisory` | Informational result exists but does not alter safety. | Apply may stay enabled. |
| `warning` | Reviewer should inspect, but domain contract allows apply. | Apply may require acknowledgement. |
| `soft_block` | Apply cannot proceed until the user resolves or acknowledges a declared condition. | Apply disabled or enabled only as `enabled_requires_ack`. |
| `hard_block` | Apply cannot proceed until validation reruns or failure resolves. | Apply disabled. |
| `policy_block` | Policy, lock, entitlement, managed admin, or deny-all posture forbids mutation. | Apply disabled; explanation action required. |
| `security_block` | Secret, trust, raw credential, unsafe route, or high-risk target rule forbids mutation. | Apply disabled; no bypass without a higher-order approval contract. |

Domain contracts MAY define stricter behavior. They may not weaken this
tiering by relabeling a policy block as warning or a stale external
probe as current.

## Lock And Policy Explanation

Locked controls are not generic disabled controls. Every locked or
policy-constrained field row MUST carry:

- `lock_state`;
- a typed `reason_class`;
- the policy or authority source ref;
- a reviewable explanation;
- a route or action to inspect policy details when one exists; and
- a support-export-safe representation of the affected target.

If policy state is stale or offline, the explanation still renders. It
must say whether the behavior is last-known-good, local-only
continuation, paused until refresh, or blocked because the source is
past grace or verification failed.

## Skipped Validation

Skipped is not success. Skipped probes MUST say who or what skipped the
probe:

- user skipped a non-required check;
- policy forbids the probe;
- offline posture prevents the probe;
- target type is unsupported;
- external provider is unavailable; or
- the flow is observe-only.

Apply may proceed with skipped validation only when the probe row says
`apply_action_may_proceed_when_skipped = true` and the staged review
names `skipped_allowed_with_reason`. Required mutation-class probes
that were skipped default to blocking.

## Surface Mapping

The shared state model maps to common form families as follows.

| Surface | Required mapping |
|---|---|
| Settings | Reuse effective-setting source, write-intent, preview, restart, checkpoint, and policy lock fields. Broad or trust-affecting writes are staged. |
| Connection setup | Probe target identity, brokered credential handle, transport posture, write posture, and schema freshness. Raw connection strings and secrets never appear. |
| Package actions | Use package review packet as domain preview; this contract supplies the shared form rollup and stale / skipped probe status. |
| Policy edits | Policy simulation / diff is the preview; policy source, owner, and stale / offline state remain visible before apply. |
| Repair actions | Repair preview, checkpoint, reversal class, and forbidden-action assertions are shown through the staged review record. |
| Network / transport | Proxy, trust-store, mirror, offline / deny-all, route exposure, and policy source are probe targets with explicit freshness. |
| Request / runtime | Environment, origin, auth/session, schema snapshot, assertion suite, replay posture, and runtime target all use probe rows. |
| Secret-bearing fields | Values are handles, aliases, redacted shapes, or class labels. Raw secret material is never a validation payload. |

## Export And Handoff

Support bundles, issue handoff packets, admin handoff exports, CLI JSON,
and review evidence MUST preserve the machine-readable state. They may
redact payload values, but they must not drop:

- form / staged review id;
- validation rollup state;
- mutation-blocking tier;
- probe source and version;
- target class and scope class;
- freshness class and capture time;
- stale / skipped decision;
- lock / policy explanation refs;
- apply action availability; and
- evidence refs and omitted-data classes.

This is the minimum that lets a support engineer or issue recipient
reconstruct why a user could see "Apply", why it was disabled, or why a
stale / skipped result was admitted.

## Conformance

A surface conforms when:

- users can distinguish current, stale, pending, failed, skipped, and
  policy-blocked validation before apply;
- required mutation-class apply never proceeds on pending, failed,
  stale-beyond-grace, skipped-required, policy-blocked, or
  security-blocked validation unless a domain contract explicitly
  admits a narrower exception;
- stale and skipped admission is explicit in both probe and staged
  review records;
- lock and policy explanations name the source and next action;
- raw secrets and raw external target payloads stay out of validation
  exports; and
- support / issue handoff preserves attribution and freshness.

The fixture corpus is the executable review checklist for this
revision. Any new enum value or behavior branch added to the schemas
must add or update a fixture in the same change.
