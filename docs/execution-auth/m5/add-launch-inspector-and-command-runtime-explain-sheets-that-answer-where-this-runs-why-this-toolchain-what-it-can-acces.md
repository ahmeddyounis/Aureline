# Launch-inspector and command-runtime explain sheets

This document is the canonical contract for the M5 **launch-inspector explain
sheet**: the export-safe, operator-facing projection that answers, for one
invocation route, *where this runs*, *why this toolchain*, *what it can access*,
and *who approved it*. Where the runtime-authority matrix states what *may* be
granted per executing surface, the execution-surface resolution packet states
which sandbox profile and toolchain back-end a surface resolves to, and the
capability-envelope packet states the concrete authority issued, an explain
sheet renders those three truths as the four questions an operator actually asks
before trusting a launch. Desktop, command, policy, CLI/headless, diagnostics,
support-export, help/About, and release surfaces consume one sheet object
instead of cloning per-surface approval or capability prose.

- Implementation: `crates/aureline-policy/src/add_launch_inspector_and_command_runtime_explain_sheets_that_answer_where_this_runs_why_this_toolchain_what_it_can_acces/`
- Boundary schema: `schemas/execution-auth/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces.md`
- Narrowed fixtures: `fixtures/execution-auth/m5/add-launch-inspector-and-command-runtime-explain-sheets-that-answer-where-this-runs-why-this-toolchain-what-it-can-acces/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_launch_inspector_explain_sheets`

## Track invariant

No ambient privilege. No AI, recipe, extension, browser-routed, or remote helper
self-issues authority: every sheet for a helper route carries an externally
issued lineage and is flagged `self_issued_by_executor: false`. The four answers,
the sandbox profile, the secret scope, the policy epoch, the approval ticket, and
the degraded reason stay inspectable and export-safe. Raw secret material,
credential bodies, and live ticket signatures stay outside the support boundary.
If an explainability packet is missing, stale, or unsupported on the platform,
the sheet **degrades to an explicit partial or unsupported label that still
carries all four answers and a named reason** — it never silently omits an
authority fact.

## Launch routes

Every invocation route gets one explain sheet; a packet missing any route fails
validation (`required_route_missing`):

| Route | Representative surface | Notes |
| --- | --- | --- |
| `desktop` | `notebook_kernel` | Operator-driven, on-device. |
| `cli` | `scaffold_hook` | Operator-driven on the headless runner. |
| `ai` | `ai_tool` | Helper route; externally issued lineage required. |
| `recipe` | `recipe` | Helper route; externally issued lineage required. |
| `extension` | `request_api_send` | Helper route; brokered network egress. |
| `remote` | `remote_mutation` | Helper route; off-device, isolated remote runtime. |
| `companion` | `browser_routed_action` | Off-device; routed from a paired companion device. |

`ai`, `recipe`, `extension`, and `remote` are **helper routes**: they may never
self-issue authority (`self_issued_authority_forbidden` otherwise). `remote` and
`companion` are **off-primary-device** routes: they must carry a verified target
identity (`off_device_target_unverified` otherwise) and preserve the identical
sheet shape even when execution is brokered by another runtime.

## What a sheet answers

Each `M5LaunchExplainSheet` carries four answer sections:

| Section | Answers | Fields |
| --- | --- | --- |
| `where_it_runs` | *Where this runs* | executing surface, sandbox profile, execution back-end, platform, profile-resolution status, `off_device`, verified `target_identity`, and a one-line `isolation_label`. |
| `why_this_toolchain` | *Why this toolchain* | `toolchain_label`, `backend_class`, `selection_reason`, and the export-safe `resolved_from_ref`. |
| `what_it_can_access` | *What it can access* | `granted_capability_classes` (always a subset of the matrix row), `allowed_scope_labels` (the allowed roots/sinks/endpoints), `secret_scope`, and the `capability_envelope_ref` it projects. |
| `who_approved_it` | *Who approved it* | `approval_posture`, `approval_ticket_ref`, `issuer_label`, `policy_epoch_label`, the ordered `decision_chain`, and `self_issued_by_executor: false`. |

A sheet also carries its overall `status` (`complete`, `partial_degraded`, or
`unsupported_on_platform`), an optional `degradation` reason block, any
`applied_downgrade_triggers`, and an optional `unsupported_profile_behavior`.

## Relationship to the upstream packets

The explain-sheet packet **consumes** the three upstream runtime-authority
truths directly rather than restating them:

- `granted_capability_classes` MUST be a subset of the frozen matrix row's
  `allowed_capability_classes` (`capability_widens_beyond_matrix` otherwise).
- `sandbox_profile` MUST be the matrix row's `default_sandbox_profile` or the
  fully inert `inert_no_execution` fail-closed profile (`sandbox_profile_widens`
  otherwise).
- the resolved surface MUST be covered by the frozen capability-envelope packet
  (`envelope_reference_uncovered` otherwise).

A sheet can therefore only ever **narrow** what the upstream packets authorize;
it can never widen them.

## Degraded and unsupported behavior

Missing, stale, or unsupported launches never drop authority facts; they relabel
the sheet and keep answering:

- `complete` — full authority; no `degradation` and no
  `applied_downgrade_triggers`, with `profile_resolution_status: supported`
  (`status_inconsistent` otherwise).
- `partial_degraded` — the launch is narrowed; the sheet carries a named
  `degradation` reason and explanation (`degraded_reason_missing` /
  `degraded_explanation_missing` otherwise) and still answers all four questions.
- `unsupported_on_platform` — the sandbox profile is unsupported on this
  platform; the sheet carries an explicit `unsupported_profile_behavior`
  (`unsupported_behavior_missing` otherwise) plus a `degradation` reason, and
  still answers all four questions.

`proof_freshness.auto_narrow_on_stale` records that a stale or missing proof
packet automatically narrows the affected sheets to a partial / unsupported label
instead of omitting authority facts.

## Enforced invariants

`M5LaunchInspectorPacket::validate` returns stable violation tokens. A packet is
rejected (and the row cannot publish) when:

- `wrong_record_kind` / `wrong_schema_version` / `missing_identity` — packet
  header is malformed.
- `missing_source_contracts` — the schema, doc, and upstream capability-envelope,
  surface-resolution, and runtime-authority matrix / issuer / approval-ticket
  contracts are not all referenced.
- `required_route_missing` — a launch route has no explain sheet.
- `sheet_incomplete` — a sheet is missing identity fields.
- `where_answer_incomplete` / `why_answer_incomplete` / `what_answer_incomplete`
  / `who_answer_incomplete` — one of the four answers omits a required fact.
- `capability_widens_beyond_matrix` / `sandbox_profile_widens` — a sheet grants
  more than its matrix row allows.
- `self_issued_authority_forbidden` — a helper route self-issues authority.
- `elevated_capability_without_ticket` — an elevated capability is granted
  without an externally issued approval-ticket ref.
- `secret_scope_inconsistent` — the secret scope and granted capabilities
  disagree.
- `off_device_target_unverified` — an off-device sheet binds to an unverified
  target identity without an explicit unverified-identity downgrade.
- `degraded_reason_missing` / `degraded_explanation_missing` / `status_inconsistent`
  / `unsupported_behavior_missing` — a degraded or unsupported sheet drops its
  explicit reason, or a complete sheet carries one.
- `envelope_reference_uncovered` — the resolved surface has no capability
  envelope.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — a required review block is unsatisfied.
- `raw_boundary_material_in_export` — the export carries forbidden secret
  material.

## Narrowed and unsupported fixtures

The fixtures exercise the failure/recovery and unsupported-platform paths and all
validate clean:

- `ai_tool_ticket_expired_partial.json` — an AI-route launch whose ticket expired
  narrows to a sanitized preview (`partial_degraded`, `approval_ticket_expired`).
- `remote_profile_unsupported_headless.json` — a remote-route launch whose
  isolated remote runtime is unsupported on headless CI fails closed
  (`unsupported_on_platform`, `fail_closed_unsupported`).
- `companion_stale_proof_partial.json` — a companion-route launch whose
  explainability proof is stale narrows to read-only (`partial_degraded`,
  `stale_proof_packet`).

## Consumer parity

The trust review and consumer projection blocks assert that desktop,
command/policy, CLI/headless, support-export, diagnostics, help/About, and
release-evidence surfaces all project the same sheets, and that every launch
route projects the identical four answers and degraded reasons. Downstream
surfaces ingest the support export directly rather than cloning explain-sheet
prose.
