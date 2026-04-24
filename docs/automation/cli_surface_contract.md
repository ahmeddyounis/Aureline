# CLI / headless JSON-surface, machine-output stability, and automation compatibility contract

This document freezes the CLI / headless JSON-surface and
machine-output stability contract Aureline publishes alongside its
command, shareability, and support-packet contracts. It is the
authority that separates **human-readable CLI output** (TTY text,
colorised tables, progress indicators) from **machine-readable
CLI output** (schema-governed JSON, NDJSON event streams, SARIF,
JUnit, TAP, LCOV, SBOM, benchmark JSON), names which commands or
export packets are **stable**, **preview**, **experimental**, or
**internal**, and pins how machine output evolves — by **versioned
schema**, not by human-output scraping or undocumented exit-code
drift.

Companion artifacts:

- [`/schemas/automation/cli_output_registry_entry.schema.json`](../../schemas/automation/cli_output_registry_entry.schema.json)
  — machine-readable boundary schema. Every CLI / headless command,
  diagnostic export, route-info emitter, build-target-discovery
  packet, and benchmark / report producer publishes exactly one
  `cli_output_registry_entry_record` against this schema.
- [`/artifacts/automation/cli_command_rows.yaml`](../../artifacts/automation/cli_command_rows.yaml)
  — seeded registry rows binding each example surface to a
  `command_id`, an authority class, an output schema, an exit-code
  register, `--dry-run` / `--explain` expectations, adapter rows
  (JUnit, SARIF, SBOM, benchmark JSON), a deprecation policy, and
  a parity-surface set desktop and support surfaces compare against.
- [`/fixtures/automation/cli_output_cases/`](../../fixtures/automation/cli_output_cases)
  — worked example machine-output cases for command execution,
  diagnostics / support export, route info, build-target discovery,
  and benchmark / report automation.

Cross-linked contracts already in the repository:

- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  and
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  — the command descriptor this registry row extends. The
  `canonical_verb`, `command_id`, `command_revision_ref`,
  `typed_arguments`, `capability_scope_class`, `preview_class`,
  `approval_posture_class`, `client_scopes`, and `docs_help_anchor_ref`
  fields re-exported into a CLI registry row originate there and
  are never re-decided here.
- [`/docs/commands/shareability_and_automation_contract.md`](../commands/shareability_and_automation_contract.md)
  and
  [`/schemas/commands/shareability_metadata.schema.json`](../../schemas/commands/shareability_metadata.schema.json)
  — `cli_equivalent_presence_class`, `headless_mode_class`,
  `automation_safety_cue`, and the why-unavailable parity rule
  originate there. This contract freezes the **output** side of the
  CLI equivalence; the shareability contract freezes the **invocation
  and discovery** side. A row that sets `cli_equivalent_presence_class
  = no_cli_equivalent_ui_only` in the shareability record MUST NOT
  appear in this CLI output registry.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — the diagnostic / support export packets the
  `diagnostics_support_export` family rows emit. This contract
  names the CLI / headless door that produces the packet; the
  packet itself is governed by the support-packet index.
- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  — the symbol / source-map / crash / generated-source / coverage /
  profile manifest the `benchmark_or_report_automation` and
  `diagnostics_support_export` rows cite by reference.
- [`/docs/release/channel_and_branch_contract.md`](../release/channel_and_branch_contract.md)
  and
  [`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml)
  — the `cli` artifact family's versioning rule and skew window. A
  CLI row's deprecation window is a projection of the `cli`
  artifact-family row; this document references that projection,
  it does not re-decide it.
- [`/docs/commands/command_parity_diff.md`](../commands/command_parity_diff.md)
  — the parity audit that consumes `parity_surface_rows`
  mechanically.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` — CLI / headless automation requirements,
  machine-output stability MUST rules, exit-code-drift MUST-NOT rules,
  `--dry-run` / `--explain` MUST rules for high-blast commands.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — CLI /
  headless surface architecture, automation compatibility posture,
  and adapter-export families (SARIF, JUnit, TAP, SBOM, benchmark
  JSON, NDJSON event streams).
- `.t2/docs/Aureline_Technical_Design_Document.md` — machine-output
  schema-binding model, adapter emitter design, exit-code class
  vocabulary.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — human TTY posture,
  stream ownership (stdout / stderr / --out-file), progress affordance
  posture.

## Why freeze this now

Without this contract, automation-facing CLI / headless output would
drift along three well-known failure modes every IDE-class product
hits sooner or later:

- **Human-output scraping.** Macros, CI jobs, editor extensions, and
  AI agents would scrape human-readable tables and progress lines,
  then silently break on every text tweak, colour tweak, or
  localisation change. The contract freezes that human TTY output
  is **not** a stable automation input; machine consumers MUST read
  the schema-governed machine envelope instead.
- **Exit-code drift.** Each subcommand would mint its own error-code
  convention ("2 for invalid arg here, 4 for invalid arg there,
  occasional bare `exit 1` everywhere"). The contract freezes a
  closed `exit_code_class` vocabulary and pins the numeric code per
  row so shell consumers and CI adapters never have to translate
  per-subcommand folklore.
- **Schema drift without versioning.** A JSON payload's shape would
  change between releases without an advertised schema or a support
  window. The contract freezes a `machine_output_stability_class`
  vocabulary, a mandatory `output_schema_binding` for every
  stable / preview / experimental row, a mandatory deprecation
  policy, and a **silent-forward-forbidden** outcome so a
  deprecated verb cannot quietly route to its successor without
  warning automation consumers.

It also freezes the **desktop / CLI / support** parity rule: a
user's palette command, an IDE context-menu entry, a CLI verb, an
AI tool handle, and a support-bundle exporter that all "do the same
thing" MUST share a `command_id`, an `authority_class`, and a
`deprecation_policy_class`. A parity audit between those surfaces
MUST match field-for-field.

## Scope

Frozen at this revision:

- One `cli_output_registry_entry_record` shape that extends the
  command descriptor with the CLI / headless surface class, the
  command family, the machine-output stability class, the machine-
  output envelope, the primary output-schema binding, the exit-code
  register, `--dry-run` / `--explain` expectations, adapter rows
  for machine-consumable exports (JUnit, SARIF, TAP, LCOV, SPDX /
  CycloneDX SBOM, benchmark JSON, NDJSON event streams), a
  deprecation-policy block, and a parity-surface set.
- A closed exit-code class vocabulary and its per-row exit-code
  register binding each class to a single numeric code.
- Schema-level couplings enforced through `if` / `then` blocks:
  every stable / preview / experimental row carries a non-null
  `schema_ref`; every high-blast row requires `--dry-run` and
  `--explain`; every `ci_exporter_only` row pins
  `file_out_owns_machine_output_stdout_reserved_for_human` stream
  ownership; every `headless_agent_machine_only` row forces
  `no_human_output_headless_only` posture; every deprecated row
  forbids silent forwarding to its successor.
- A registry-level invariants block pinned to `true` at schema
  level so a seed that elides any of the nine registry invariants
  is rejected at validation time.
- Example worked rows for the five families named in the
  acceptance criteria: command execution, diagnostics / support
  export, route info, build-target discovery, and benchmark /
  report automation.

Out of scope until a superseding decision row opens:

- The live CLI router, the live adapter emitters, and the live
  round-trip conformance harness. This contract names the schema
  refs and fixture refs those harnesses will consume; they land in
  their own lanes.
- Full subcommand breadth. The seed binds the **five families** the
  acceptance criteria name explicitly; additional commands land as
  they come online, each as one row against this schema.
- The wire protocol for long-running headless invocations (progress
  event frames, cancellation tokens). The envelope class
  (`ndjson_event_stream`) is frozen here; the event schema is its
  own boundary.
- The flag spellings for `--dry-run`, `--explain`, `--format`,
  `--out-file`, and adapter selection. Flag spellings resolve
  through docs-pack anchors; this contract carries the opaque ref
  only.

## Human-readable vs machine-readable output

The first and most important cut this contract makes:

| Axis                                     | Human-readable CLI output                                    | Machine-readable CLI output                                              |
|------------------------------------------|--------------------------------------------------------------|--------------------------------------------------------------------------|
| Consumer                                 | Humans at a terminal                                         | Macros, recipes, AI tools, CI adapters, headless automation              |
| Stability contract                       | Not stable. May change across any release.                   | Schema-governed. Stability class pinned per row.                         |
| Versioning                               | None.                                                        | `schema_version` field inside the payload; additive-minor vs breaking.   |
| Localisation                             | Localised per user locale.                                   | Locale-fixed (`en_US` or POSIX `C`).                                     |
| Colour and TTY escapes                   | Permitted and encouraged.                                    | Forbidden. No ANSI escapes in the machine envelope.                      |
| Progress indicators                      | Permitted on stderr or an in-place overlay.                  | Separated: progress on stderr or an NDJSON event stream, never mixed with the primary machine payload. |
| Automation scrape?                       | **Non-conforming**. Automation MUST NOT scrape human output. | Supported. Automation reads the schema-bound envelope directly.          |

The `human_output_posture_class` slot on every row pins which
posture the row takes. The default for CLI rows is
`human_tty_not_stable`; the row explicitly declares whether the
human rendering is permitted at all
(`human_tty_interactive_only_no_machine_output`), is a mirror of
the machine envelope (`human_tty_mirror_of_machine_output`), or is
absent because the surface is headless-only
(`no_human_output_headless_only`).

## Machine-output stability vocabulary

`machine_output_stability_class` is the closed vocabulary every
CLI help surface, AI-tool descriptor, docs reference page, support
export, and shiproom row reads:

| Stability class                                         | Meaning                                                                                                                                                                                                                          |
|---------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `stable_schema_governed`                                | Schema is frozen. Additive-minor bumps are permitted; breaking changes require a new decision row and a support-window transition. External automation may depend on the row without a support waiver.                          |
| `preview_schema_governed_additive_minor_only`           | Schema is published but explicitly subject to additive-minor churn. Breaking changes are still decision-row-gated, but the row advertises that fields may appear between releases. Automation may depend on the row with eyes open. |
| `experimental_schema_governed_may_break`                | Schema is published, but the row is explicitly permitted to break between releases. Reserved for early surfaces where the shape is still being learned. Automation consumers MUST treat breakage as expected.                 |
| `internal_no_stability_promise`                         | No external stability promise. Permitted only on surfaces scoped to the running Aureline process itself (internal debug envelope, developer-facing introspection). Adapter rows are restricted to `internal_debug_envelope`.      |

**Schema-enforced couplings.** Every stable, preview, or
experimental row MUST carry a non-null `schema_ref` in its
`output_schema_binding`; the schema's `if` / `then` block rejects
rows that omit it. Every stable row MUST carry a non-null
`deprecation_policy`; the schema's policy block enforces the
successor pointer / support window rules per policy class.

## Machine-output envelope vocabulary

The `machine_output_envelope_class` names the single primary
envelope a machine consumer binds against:

- `json_document_single` — one JSON document on stdout (or on the
  file named by `--out-file` when the row's stream ownership is
  `file_out_owns_machine_output_stdout_reserved_for_human`).
- `jsonl_line_stream` — newline-delimited JSON records, one per
  line, each record validated against the bound schema.
- `ndjson_event_stream` — newline-delimited JSON event frames for
  long-running commands; frames carry a discriminator field and
  resolve against the event-stream frame schema named in
  `output_schema_binding`.
- `sarif_2_1_0_document` — SARIF 2.1.0 static-analysis results.
- `junit_xml_document` — JUnit XML test report (wrapped or
  unwrapped as the CI runner expects).
- `tap_14_stream` — TAP 14 test results.
- `lcov_coverage_document` — LCOV coverage.
- `spdx_sbom_document` / `cyclonedx_sbom_document` — SBOM exports.
- `benchmark_json_document` — benchmark / report JSON document.
- `log_only_human_not_stable` — reserved for rows whose human
  posture is `human_tty_interactive_only_no_machine_output`; no
  machine envelope is promised.

The adapter vocabulary (`adapter_class`) lets a single row declare
**additional** machine-consumable exports the same command can
produce on demand — e.g. a test runner whose primary envelope is
`json_document_single` can also expose a `junit_xml_test_report`
adapter and a `sarif_2_1_0_analysis_results` adapter, each bound
to its own schema and invocation flag.

## Exit-code registry

Every row carries a non-empty `exit_code_register`. The closed
`exit_code_class` vocabulary is:

| Class                                    | Numeric code rule                                                        | Notes                                                                                               |
|------------------------------------------|--------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------|
| `success`                                | MUST be `0`.                                                             | The single success path.                                                                            |
| `success_no_action_taken`                | MUST be `0`.                                                             | Idempotent re-run or already-at-target state.                                                       |
| `partial_success_with_warnings`          | Non-zero per row choice.                                                 | Row MUST bind a `warnings_payload_schema_ref`.                                                      |
| `usage_error`                            | Non-zero per row choice (recommend `2`).                                 | Invalid argv, unknown subcommand, malformed flag.                                                   |
| `input_validation_error`                 | Non-zero per row choice.                                                 | Input parsed but failed schema validation or business-rule validation.                              |
| `policy_or_trust_denied`                 | Non-zero per row choice.                                                 | ADR 0001 trust or ADR 0008 admin-policy denied the invocation.                                      |
| `credential_broker_denied`               | Non-zero per row choice.                                                 | ADR 0007 broker refused the handshake.                                                              |
| `preview_required_not_shown`             | Non-zero per row choice.                                                 | Row requires preview and the CLI surface cannot render one (e.g. `--explain` missing on high-blast).|
| `approval_required_not_granted`          | Non-zero per row choice.                                                 | Row requires approval and approval was not supplied in the invocation session.                      |
| `dry_run_would_have_applied`             | Non-zero per row choice.                                                 | Row ran under `--dry-run` and the planned effect was non-trivial.                                   |
| `timeout_or_deadline_exceeded`           | Non-zero per row choice.                                                 | Deadline exceeded.                                                                                   |
| `network_or_remote_unavailable`          | Non-zero per row choice.                                                 | Remote-read or remote-mutation surface could not reach the remote.                                  |
| `kill_switch_active`                     | Non-zero per row choice.                                                 | ADR 0011 kill switch denied the invocation.                                                          |
| `dependency_missing_or_stale`            | Non-zero per row choice.                                                 | Required dependency (symbol server, extension, remote agent) missing or stale.                      |
| `unsupported_on_headless`                | Non-zero per row choice.                                                 | UI-only command attempted from headless; forces `ui_only_interactive` cue.                          |
| `cancelled_by_user`                      | Non-zero per row choice.                                                 | User cancelled interactively or via signal.                                                          |
| `unrecoverable_internal_error`           | Non-zero per row choice.                                                 | Bug bucket; row MUST cite a support-bundle export path in `meaning_ref`.                             |

External automation reads the **class**, not the number. The number
is pinned so shell consumers and CI adapters have stable behaviour;
changing a row's numeric code for an existing class is breaking.

## `--dry-run` and `--explain` expectations

Every row declares `dry_run_support_class` and `explain_support_class`.
The schema enforces that irreversible local mutation, remote
mutation, and high-blast rows set `dry_run_required_for_high_blast`
(or `dry_run_not_supported_irreversible_under_approval_only` when
approval gates the action) **and** `explain_required_for_high_blast`.

- `--dry-run` is the non-interactive equivalent of the preview pane.
  It MUST produce the same machine envelope the real invocation
  would produce but MUST set a planned-but-not-applied marker in
  the payload, and MUST exit with `dry_run_would_have_applied` when
  the plan is non-trivial.
- `--explain` is the non-interactive equivalent of the preview
  pane's "why" surface. It MUST emit a structured explanation
  payload the human preview narrates from, MUST NOT apply any
  side-effect, and MUST exit with `success` when the explanation
  was rendered.

Read-only surfaces (route info, build-target discovery) set
`dry_run_not_applicable_read_only` and
`explain_not_applicable_read_only`; there is no "dry-run read".

## Adapter rows for machine-consumable exports

`adapter_rows` lets a row declare additional machine-consumable
exports the same command produces on demand. The adapter vocabulary
is drawn from industry-standard formats where they exist:

- `junit_xml_test_report` — JUnit XML for CI runners.
- `sarif_2_1_0_analysis_results` — SARIF 2.1.0.
- `tap_14_test_results` — TAP 14.
- `lcov_coverage_report` — LCOV.
- `spdx_sbom_export` / `cyclonedx_sbom_export` — SBOM.
- `benchmark_json_export` — benchmark / report JSON.
- `ndjson_event_stream_export` — NDJSON event stream companion
  (for a primary single-document row that also exposes an event
  stream companion).
- `otlp_trace_export` — OpenTelemetry OTLP traces.
- `internal_debug_envelope` — reserved for internal / non-stable
  rows.

Every adapter row carries its **own** `schema_ref` and its **own**
`stability_class`, so a row whose primary envelope is
`stable_schema_governed` can expose a `preview_schema_governed`
SARIF adapter alongside the stable primary JSON.

## Deprecation and compatibility rules

`deprecation_policy` is a block, not a single enum. Every row
carries it. The admissible policy classes are:

- `not_deprecated` — the row is live; `outcome_class` is pinned to
  `silent_forward_to_successor_forbidden` as a defensive constant
  (there is no successor to forward to).
- `deprecated_with_successor` — the row is deprecated. Schema
  requires a non-null `successor_command_id`, a non-null
  `support_window_class` (`one_stable_train`, `two_stable_trains`,
  or `one_lts_train`), and an `outcome_class` drawn from the
  emitting subset (`deny_with_successor_pointer`,
  `warn_once_and_succeed`, `warn_every_invocation_and_succeed`).
  Silent forwarding to the successor is forbidden — automation
  consumers MUST be able to detect the drift.
- `retired_migration_bridge_only` — the row is retired. Schema
  pins the outcome to `deny_with_successor_pointer` and the
  support window to `bridge_only_no_support_window`; the row
  surfaces only in migration bridge cards and help search.
- `deprecation_not_applicable_internal` — reserved for
  `internal_no_stability_promise` rows.

**Support window coupling.** The CLI artifact family in
[`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml)
owns the actual support-window duration for the CLI surface; the
`support_window_class` slot here is the per-row projection. A
deprecation policy whose window is narrower than the CLI family's
window is non-conforming.

**No silent forwarding.** This is the single rule that keeps
machine consumers honest. A deprecated CLI verb MUST emit either
a typed deny (with the successor `command_id`) or a typed warning
(on every invocation or once per session); it MUST NOT route to
the successor without telling the caller. The schema pins
`no_silent_forward_on_deprecated_verbs = true` at the registry
invariants block.

## Parity with desktop and support surfaces

Every row's `parity_surface_rows` names the desktop and support
surfaces that MUST resolve the same command through the shared
`command_id`. `match_mode` is one of:

- `exact_match_required` — `command_id`, `authority_class`, and
  `deprecation_policy.policy_class` MUST match field-for-field.
- `projection_match_required` — the surface projects the row via a
  registry-owned projection; label text and shortcut rendering may
  differ but the three pinned fields MUST match.
- `informational_only` — the surface cites the CLI verb in prose
  (e.g. a docs reference page) without owning a parity cell.

A parity audit between a palette row and the CLI row with
`exact_match_required` that disagrees on `command_id`,
`authority_class`, or `deprecation_policy.policy_class` is a
registry violation, not a narrative disagreement; the parity
validator rejects it.

## Example families

The seed binds one row per family named in the acceptance criteria.
Each family row cites at least one fixture under
[`/fixtures/automation/cli_output_cases/`](../../fixtures/automation/cli_output_cases)
demonstrating the machine envelope, the exit-code path, and the
adapter output where applicable.

| Family                                  | Example row                                   | Machine envelope                  | Adapter rows declared            | Fixture                                                                  |
|-----------------------------------------|-----------------------------------------------|-----------------------------------|----------------------------------|--------------------------------------------------------------------------|
| Command execution                       | `workspace_open_folder_cli_execution`         | `json_document_single`            | `ndjson_event_stream_export`     | `command_execution_workspace_open_folder.json`                          |
| Diagnostics / support export            | `support_export_bundle`                       | `json_document_single`            | `spdx_sbom_export` (preview)     | `diagnostics_support_export_bundle.json`                                |
| Route / invocation info                 | `route_info_introspection`                    | `json_document_single`            | none                             | `route_info_introspection.json`                                         |
| Build-target discovery                  | `build_target_discovery`                      | `json_document_single`            | `cyclonedx_sbom_export`          | `build_target_discovery.json`                                           |
| Benchmark / report automation           | `benchmark_report_emit`                       | `benchmark_json_document`         | `junit_xml_test_report` (preview), `sarif_2_1_0_analysis_results` (preview), `ndjson_event_stream_export` | `benchmark_report_emit.json` |

## Registry-level invariants

The schema's `registry_invariants_block` pins nine constants to
`true`. A seed that sets any of them to `false` is non-conforming;
the block is how this contract freezes its MUST rules
mechanically rather than leaving them as prose:

1. `no_stable_machine_output_without_schema_binding` — stable,
   preview, and experimental rows all carry a schema ref.
2. `no_stable_machine_output_without_exit_code_register` —
   every row carries a non-empty exit-code register.
3. `no_stable_machine_output_without_deprecation_policy` —
   every row carries a deprecation policy block.
4. `no_silent_forward_on_deprecated_verbs` — deprecated verbs
   emit typed deny / warn, never a silent route to successor.
5. `no_adapter_export_without_adapter_schema_ref` — every adapter
   row carries its own schema ref.
6. `no_human_output_scraping_promise` — human TTY output is
   never a stable automation input.
7. `every_row_cites_command_id_and_revision` — the row is joined
   to the descriptor through `command_id` + `command_revision_ref`.
8. `every_row_declares_parity_surface_set` — the row declares
   at least one parity-surface row.
9. `every_high_blast_row_requires_dry_run_and_explain` — the
   high-blast / remote-mutation coupling is enforced at schema
   level.

## Schema of record

The eventual Aureline CLI / automation crates' Rust types are the
schema of record. The JSON Schema export at
[`/schemas/automation/cli_output_registry_entry.schema.json`](../../schemas/automation/cli_output_registry_entry.schema.json)
is the cross-tool boundary every non-owning surface reads. Adding
a new `cli_surface_class`, `machine_output_stability_class`,
`machine_output_envelope_class`, `command_family_class`,
`exit_code_class`, `dry_run_support_class`, `explain_support_class`,
`authority_class`, `adapter_class`, `deprecation_policy_class`,
`deprecation_outcome_class`, `support_window_class`, or
`parity_surface_class` is additive-minor and bumps
`cli_output_registry_schema_version`; repurposing an existing value
is breaking and requires a new decision row.

## Source anchors

- [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
- [`docs/commands/shareability_and_automation_contract.md`](../commands/shareability_and_automation_contract.md)
- [`docs/commands/command_parity_diff.md`](../commands/command_parity_diff.md)
- [`docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
- [`docs/release/channel_and_branch_contract.md`](../release/channel_and_branch_contract.md)
- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — CLI / headless automation, machine-output stability,
  `--dry-run` / `--explain`, adapter family, exit-code, and
  deprecation-policy requirements.
