# Problem, Output, and Evidence Chain Contract

This document freezes the chain that ties a visible problem row to the
output, event, artifact, remap, review, support, and verification evidence
that caused it to appear. The chain is a join contract, not another
diagnostic model: language diagnostics, task-event envelopes, evidence
links, output viewers, support bundles, and release packets keep their own
records, while this packet gives every surface one stable way to cite their
relationship.

Companion artifacts:

- [`/schemas/diagnostics/problem_evidence_chain.schema.json`](../../schemas/diagnostics/problem_evidence_chain.schema.json)
  - boundary schema for one `problem_evidence_chain_record`.
- [`/schemas/diagnostics/heuristic_confidence.schema.json`](../../schemas/diagnostics/heuristic_confidence.schema.json)
  - boundary schema for the shared heuristic-confidence vocabulary and
    its required copy / downgrade behavior.
- [`/fixtures/diagnostics/problem_evidence_cases/`](../../fixtures/diagnostics/problem_evidence_cases/)
  - worked YAML fixtures for build-output linkage, imported scan plus
    local rerun, correlated multi-signal incidents, and unknown-cause
    placeholders.

This contract composes with, and does not replace:

- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  and
  [`/schemas/language/diagnostic_cluster.schema.json`](../../schemas/language/diagnostic_cluster.schema.json)
  for canonical diagnostic rows, clustering, freshness, and remap state.
- [`/docs/tooling/task_event_contract_seed.md`](../tooling/task_event_contract_seed.md)
  and
  [`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
  for task, build, test, debug, notebook, and heuristic output events.
- [`/schemas/execution/evidence_link.schema.json`](../../schemas/execution/evidence_link.schema.json)
  for typed evidence edges between command, run, output, result,
  artifact, history, and support objects.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  for governed support-bundle export and redaction behavior.
- [`/schemas/qe/public_proof_packet.schema.json`](../../schemas/qe/public_proof_packet.schema.json)
  for release and public-proof packet citations.
- `.t2/docs/Aureline_PRD.md` freshness/confidence and build-intelligence
  rules, plus `.t2/docs/Aureline_Technical_Design_Document.md`
  appendices for Problems/output/evidence, diagnostic source taxonomy,
  SARIF import, and support-packet minimums.

If this document disagrees with those sources, those sources win and this
contract, schemas, and fixtures update in the same change.

## Why Freeze This Now

Without a chain contract, every consumer is tempted to summarize a problem
differently:

- the Problems row points at a compiler message;
- the output pane points at a stream chunk;
- the support bundle points at a redacted task event;
- a release packet points at a proof packet;
- an evidence overlay points at a remap record; and
- a local diagnosis view invents a confidence label that no export can
  reproduce.

The result is multiple partial truths about the same suspected issue. This
contract keeps those surfaces on one joinable chain so support, review,
release, CLI, AI context, and local UI can cite the same `problem_chain_id`
and the same confidence vocabulary without flattening the underlying
evidence.

## Scope

Frozen at this revision:

- one `problem_evidence_chain_record` that links a problem projection to
  originating output, event, artifact, imported scan, remap, support,
  review, and release evidence refs;
- one heuristic-confidence taxonomy covering `exact`,
  `imported_authoritative`, `derived_structured`, `heuristic_parsed`,
  `correlated_suggestive`, and `unknown`;
- collapse and expansion rules for related problems when several signals
  point at the same suspected issue;
- export rules for local diagnosis views, support bundles, review packs,
  release packets, CLI JSON, and AI context packets.

Out of scope:

- the full Problems UI;
- log viewers, output viewers, or observability surfaces;
- parser implementation for any one tool;
- automatic support-bundle upload or hosted support workflow;
- proving root cause from correlation alone.

## Chain Objects

A chain is the smallest exportable object that answers:

1. Which problem row did the user see?
2. Which output, event, artifact, or import record caused that row?
3. What target, execution context, and freshness envelope was the evidence
   collected under?
4. Was the location exact, remapped, stale, imported, unmapped, or not
   applicable?
5. Which review, support, release, and local diagnosis exports can cite
   the same chain?
6. What confidence class may the surface claim without overstating source
   truth?

Every chain has a stable `problem_chain_id`. A surface may project that id
into a row id, support item id, review annotation id, or release packet ref,
but it may not mint a different local confidence story for the same
evidence.

### Required Blocks

| Block | Purpose |
|---|---|
| `problem_projection` | The visible problem row or placeholder, including rule, severity, anchor, and canonical diagnostic refs when they exist. |
| `origin_records` | Source records that caused or preserved the chain: diagnostic clusters, task-event envelopes, imported scan records, evidence links, support replay records, or unknown placeholders. |
| `linked_outputs` | Output, stream, task-event, artifact, output-viewer, notebook-output, replay-bundle, or provider-output refs that can be opened or exported. |
| `target_context` | Workspace, run, execution context, target, toolchain, environment, support, or release context that bounds the evidence. |
| `freshness` | Current, recent, stale, superseded, imported, or unknown posture for the chain as rendered. |
| `remap_state` | Exact, contextual, stale, unmapped, imported-static, not-applicable, or unknown anchor posture. |
| `confidence` | One value from the heuristic-confidence taxonomy plus required disclosure and downgrade refs. |
| `correlation_basis` | Why the chain is grouped or joined, and whether it claims causality or only correlation. |
| `evidence_refs` | Typed refs to raw-payload handles, sanitized slices, task events, diagnostics, artifacts, support bundles, release packets, imported scans, remap evidence, policy decisions, and redaction reports. |
| `collapse_projection` | Display collapse class, expanded chain refs, distinct evidence refs preserved for detail view, and user-safe copy. |
| `export_projection` | Surfaces that cite the chain id and confidence class, with redaction and omission behavior. |

## Source Classes

`source_class` names the strongest origin class the chain is allowed to
claim. It is not a UI label and it does not replace the source vocabularies
inside diagnostic clusters or task-event envelopes.

| `source_class` | Meaning | Required behavior |
|---|---|---|
| `structured_language_diagnostic` | A language, compiler, parser, linter, analyzer, or policy diagnostic emitted structured fields. | Cite the diagnostic cluster or remap record and preserve source kind, freshness, and anchor state. |
| `normalized_task_event` | A task, build, test, debug, notebook, or headless adapter emitted a structured task-event envelope. | Cite the task-event envelope and its execution context. |
| `heuristic_output_parse` | A parser inferred problem shape from unstructured output. | Cite a raw-payload or sanitized-output backlink and use downgraded confidence. |
| `imported_provider_annotation` | A provider, CI, review, or hosted system imported a problem annotation. | Preserve provider/import freshness and mapping quality; do not claim live local proof. |
| `imported_scan_or_report` | SARIF-like, SAST, secrets, IaC, license, SBOM, or compliance evidence was imported. | Preserve import session, tool/rule-pack, baseline family, and local rerun comparison separately. |
| `replayed_support_evidence` | A support bundle or replay harness reconstructed the chain. | Mark replayed origin and cite bundle or replay refs. |
| `unknown_placeholder` | The product needs a problem row or support item before cause is known. | Carry an unknown confidence class and bounded recovery action; do not invent origin truth. |

## Heuristic-Confidence Taxonomy

All surfaces use the same six classes.

| Class | Label | May Claim | Required Copy | Downgrade Behavior |
|---|---|---|---|---|
| `exact` | Exact | The cited source record directly supplied the problem fields and current anchor under the named target context. | "Exact source evidence" plus the source record ref. | Downgrade if the source record is imported, remapped, truncated, parsed from text, stale, or correlated from multiple signals. |
| `imported_authoritative` | Imported authoritative | The imported source is authoritative for its own system or scan, but not for current local truth. | "Imported evidence" plus source, import time, freshness, and mapping posture. | Downgrade if local mapping is missing, provider freshness is stale, or a local rerun conflicts. |
| `derived_structured` | Derived structured | Aureline derived the chain from structured records without parsing raw prose. | "Derived from structured records" plus the records joined. | Downgrade if required fields were inferred, lost to truncation, or joined only by weak correlation. |
| `heuristic_parsed` | Heuristic parsed | A best-effort parser matched output or text and retained a backlink. | "Heuristic parse" plus raw/sanitized output backlink and parser id. | Downgrade to `unknown` when the backlink is unavailable, redacted beyond review, or target context is missing. |
| `correlated_suggestive` | Correlated suggestive | Multiple signals suggest a shared incident or failure family. | "Correlated, not proven" plus distinct evidence refs and causality caveat. | Downgrade to `unknown` if any signal loses identity, if target contexts diverge, or if contradictory evidence is unresolved. |
| `unknown` | Unknown | A placeholder exists, but the cause, source, or mapping is not yet established. | "Cause unknown" plus next safe recovery or rerun action. | May upgrade only when a new source record is attached; never by copy change alone. |

Rules:

1. `exact` is reserved for a current source record that supplied the
   problem fields directly. It is not available to heuristic output parses
   or multi-signal correlations.
2. `imported_authoritative` is authoritative only for the imported source.
   Local surfaces must still disclose imported freshness and mapping state.
3. `derived_structured` may join structured records, but it may not hide
   the fact that the joined chain is a projection.
4. `heuristic_parsed` must always carry an output backlink or an explicit
   omission reason.
5. `correlated_suggestive` must use copy that denies proven causality.
6. `unknown` may not be styled as an actionable exact finding. It can route
   a rerun, import repair, support export, or placeholder details view.

## Collapse and Expansion Rules

Collapsing is a display choice. It never deletes chain members, evidence
refs, confidence classes, or causality caveats.

Problem chains may collapse when all of these are compatible:

1. normalized rule, failure family, or incident family;
2. target context or explicitly equivalent target context;
3. anchor/remap family, or a target-scope-only relationship that does not
   claim a file anchor;
4. freshness posture that can be shown without hiding stale or imported
   state;
5. suppression, baseline, and policy posture; and
6. safe next action.

Problem chains must remain separate, or collapse only under a blocked
review state, when any of these differ:

- one chain is exact and another is only correlated;
- one chain points at a different target, run, workspace, provider project,
  release candidate, or support export;
- one chain requires a different mutation, suppression, baseline, rerun, or
  security action;
- evidence contradicts the suspected issue;
- one chain has unknown cause and another has an established source; or
- combining them would imply a root cause that none of the records prove.

`collapse_projection.causality_claim_class` is the controlling field for
copy. A compact row may say that signals are related only when the class is
`correlated_not_proven` or weaker; it may say they are one source only when
the class is `proven_same_source` or `same_normalized_problem`.

## Export Behavior

Local diagnosis views, support bundles, review packets, release packets,
CLI JSON, and AI context packets cite the same:

- `problem_chain_id`;
- `confidence.confidence_class`;
- `source_class`;
- `freshness.freshness_class`;
- `remap_state.remap_state_class`;
- `origin_records[].record_ref`; and
- `evidence_refs[].evidence_ref`.

Export surfaces may omit raw payloads, raw source text, raw logs, raw paths,
URLs, command lines, environment bodies, and secret material according to
their redaction class, but they may not omit the chain id or silently replace
the confidence vocabulary with prose such as "likely", "confirmed", or
"maybe". If an export cannot carry an evidence body, it carries an
`omission_reason_ref` or a by-reference handle.

Support bundles must include enough chain metadata for a recipient to
reconstruct:

1. the visible problem row or placeholder;
2. the source/output records that led to it;
3. the target and freshness state at capture time;
4. the confidence class and required disclosure copy;
5. any distinct evidence hidden from the compact row; and
6. any redaction or retention reason that prevents raw replay.

Release and verification packets cite chain ids only when the chain's
evidence is within the packet's freshness and redaction policy. Stale or
unknown chains may still be included as caveats, but they must not support a
positive release claim without a rerun or explicit waiver.

## Fixture Corpus

The fixture corpus under
[`/fixtures/diagnostics/problem_evidence_cases/`](../../fixtures/diagnostics/problem_evidence_cases/)
covers:

| Fixture | Scenario |
|---|---|
| `build_output_to_problem_row.yaml` | A structured build task event emits a diagnostic and links to the visible problem row plus output viewer. |
| `imported_scan_plus_local_rerun.yaml` | Imported scan evidence stays imported while a local structured rerun provides a comparable current signal. |
| `correlated_multi_signal_incident.yaml` | A test failure, heuristic output parse, and runtime log are grouped as suggestive evidence without claiming proven causality. |
| `unknown_cause_placeholder.yaml` | A support or local diagnosis placeholder exists with unknown cause and a safe rerun/export path. |

These fixtures are normative examples. Future implementations may add more
fields or cases, but they may not weaken chain identity, confidence
honesty, remap disclosure, or export parity without updating the schemas and
this document together.
