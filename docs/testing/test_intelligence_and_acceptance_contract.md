# Test intelligence and acceptance contract

This contract freezes the acceptance semantics for coverage merges,
flaky verdicts, snapshot / golden updates, and AI-generated test
proposals. It sits above the execution test records: execution records
preserve what ran and what artifacts were produced; this contract
preserves whether downstream surfaces may treat the evidence as complete,
compatible, reviewable, or blocked.

The goal is to prevent ambiguous green states. A coverage overlay,
snapshot acceptance sheet, flaky badge, or AI test proposal must expose
scope, compatibility, missing evidence, policy limits, and review gates
before a user or release packet can rely on it.

Machine-readable companions:

- [`/schemas/testing/coverage_merge_result.schema.json`](../../schemas/testing/coverage_merge_result.schema.json)
  - the `coverage_merge_result_record`, including included artifacts,
    shards, platforms, commit / build identity, line-vs-branch support,
    partial and incompatible warnings, and the resulting coverage scope.
- [`/schemas/testing/snapshot_acceptance_review.schema.json`](../../schemas/testing/snapshot_acceptance_review.schema.json)
  - the `snapshot_acceptance_review_record`, including baseline
    identity, render mode, raw/text fallback, environment scope,
    accept/reject granularity, per-artifact decisions, and rollback /
    local-history linkage.
- [`/schemas/testing/ai_test_generation_gate.schema.json`](../../schemas/testing/ai_test_generation_gate.schema.json)
  - the `ai_test_generation_gate_record`, including requested scope,
    cited evidence, selector truth, protected-path restrictions, preview
    requirement, sandbox/run plan, and no-automatic-land behavior.
- [`/fixtures/testing/test_intelligence_cases/`](../../fixtures/testing/test_intelligence_cases/)
  - worked YAML fixtures covering partial shard merge, incompatible
    branch-coverage merge, snapshot acceptance with raw fallback, an
    AI-generated test proposal blocked on a protected path, and a flaky
    verdict transition ledger.

This contract composes with and does not replace:

- [`/docs/execution/test_truth_contract.md`](../execution/test_truth_contract.md),
  [`/schemas/execution/test_discovery_state.schema.json`](../../schemas/execution/test_discovery_state.schema.json),
  [`/schemas/execution/test_run_summary.schema.json`](../../schemas/execution/test_run_summary.schema.json),
  and
  [`/schemas/execution/flaky_history.schema.json`](../../schemas/execution/flaky_history.schema.json).
  The execution records remain the source for discovery, run summaries,
  base snapshot review, coverage handoff, flaky history, and quarantine.
- [`/docs/testing/test_item_identity_contract.md`](./test_item_identity_contract.md),
  [`/schemas/testing/test_item_identity.schema.json`](../../schemas/testing/test_item_identity.schema.json),
  and
  [`/schemas/testing/test_selector_grammar.schema.json`](../../schemas/testing/test_selector_grammar.schema.json).
  Acceptance records cite canonical identities and selector records; they
  never match display labels.
- [`/docs/testing/test_session_and_attempt_contract.md`](./test_session_and_attempt_contract.md),
  [`/schemas/testing/test_session.schema.json`](../../schemas/testing/test_session.schema.json),
  and
  [`/schemas/testing/test_attempt.schema.json`](../../schemas/testing/test_attempt.schema.json).
  Test sessions and attempts preserve selector, target, environment,
  source, artifact, imported-CI, and reconstruction truth for the
  acceptance layer.
- [`/docs/testing/test_quarantine_and_mute_contract.md`](./test_quarantine_and_mute_contract.md)
  and
  [`/schemas/testing/quarantine_record.schema.json`](../../schemas/testing/quarantine_record.schema.json).
  Release-facing quarantine, mute, stable-again, owner, expiry, and
  release-packet treatment stay there. This contract freezes the
  verdict vocabulary and transition gates that feed those rows.
- [`/docs/ai/evidence_replayability_contract.md`](../ai/evidence_replayability_contract.md),
  [`/docs/ai/context_assembly_contract.md`](../ai/context_assembly_contract.md),
  and
  [`/docs/editor/refactor_and_replace_transaction_contract.md`](../editor/refactor_and_replace_transaction_contract.md).
  AI test-generation proposals cite retained evidence and produce
  previewable patch / transaction refs rather than landing changes
  directly.

Raw command lines, raw stdout or stderr bytes, raw environment bodies,
raw absolute paths, raw URLs, raw secret values, raw test names, raw
assertion bodies, raw source excerpts, raw snapshot bytes, raw coverage
payloads, raw provider payloads, raw AI prompt text, raw generated test
source, and raw stack traces MUST NOT cross these boundaries. Records
carry opaque refs, hashes, counts, bounded summaries, timestamps, and
closed vocabulary values only.

## Coverage merge results

The `coverage_merge_result_record` is the reviewable result of merging
coverage artifacts from one or more test attempts, provider imports, CI
shards, or platform-specific runs. It does not own the coverage bytes;
coverage payloads remain on the artifact rail. The merge result tells a
surface what was included, what was omitted, which metric families are
compatible, and what scope can be claimed.

Every coverage merge result MUST publish:

| Field group | Required content | Rule |
|---|---|---|
| Identity | merge result id, schema version, created time, merge basis, producer refs | A merge result is immutable; later re-merges mint new rows. |
| Source linkage | test session refs, attempt refs, run-summary refs, coverage-handoff refs, artifact refs | The record cites existing execution evidence and never re-mints coverage bytes. |
| Included inputs | artifacts, shards, platforms, target environments, commit/build/toolchain refs, coverage format, metric support | Every counted artifact must be named with its identity and support class. |
| Omitted inputs | omitted shard refs, omitted platform refs, omitted artifact refs, omission reason | Missing evidence is visible and cannot be collapsed into full scope. |
| Compatibility | line support, branch support, metric support, source-map or generated-source mapping state, format compatibility | Branch coverage missing or incompatible is not the same as branch coverage zero. |
| Warnings | partial, stale, incompatible, mixed-build, missing-shard, omitted-platform, unsupported-metric warnings | Any warning prevents a clean complete state. |
| Resulting scope | workspace/workset/slice/test scope, included roots, omitted roots, changed-file scope, platform scope, file/line/branch counts | Surfaces can explain exactly what the merged number covers. |
| Display posture | review state, summary label, release use, downgrade class | A partial or incompatible merge cannot render as an unqualified green badge. |

Closed merge-completeness vocabulary:

| Value | Meaning |
|---|---|
| `complete_exact_scope` | All expected shards, platforms, and metric families for the requested scope were included and compatible. |
| `partial_missing_shards` | At least one expected shard was absent or unreadable. |
| `partial_omitted_platforms` | At least one requested platform/target was omitted. |
| `partial_redaction_or_policy_limited` | Policy or redaction prevented a full merge. |
| `incompatible_metrics_excluded` | Some artifacts were excluded because metric support or format was incompatible. |
| `incompatible_branch_coverage_excluded` | Line coverage may be usable, but branch coverage from at least one input was incompatible or excluded. |
| `no_compatible_artifacts` | Inputs existed, but none could be safely merged. |
| `merge_completeness_unknown_requires_review` | The merge cannot be classified; fail closed. |

Rules:

1. `complete_exact_scope` is allowed only when no warning of class
   `missing_shard`, `omitted_platform`, `stale_artifact`,
   `source_revision_mismatch`, `build_identity_mismatch`,
   `incompatible_format`, `incompatible_metric_family`,
   `incompatible_branch_coverage`, `policy_limited_scope`, or
   `redaction_limited_scope` is present.
2. Line and branch support are separate fields. A line-compatible merge
   with branch incompatibility may claim line coverage for the resulting
   scope, but it MUST NOT claim branch coverage completeness.
3. Imported provider artifacts and cached local artifacts remain
   attributable. A merge that combines imported and local evidence MUST
   cite the parity or comparability records that justify the merge.
4. Coverage for changed files is a narrowed scope. It may support review
   risk assessment, but it cannot imply full workspace, package, module,
   or branch coverage.
5. A missing shard, omitted platform, stale artifact, or incompatible
   metric family must be listed in `warnings` and in the appropriate
   omitted-input collection. Hiding it in prose is non-conforming.

## Flaky verdict vocabulary

Flaky verdicts are controlled terms shared by editor badges, test trees,
CLI summaries, support exports, AI triage, quarantine rows, scorecards,
and release evidence. They are not free-form labels.

| UI/export label | Schema value | Meaning | Required evidence |
|---|---|---|---|
| `Suspected` | `suspected` | Intermittent or suspicious behavior was observed, but comparable reproduction is incomplete. | At least one observed signal, imported signal, or cited AI inference anchor; not enough to quarantine automatically. |
| `Reproduced` | `reproduced` | Comparable reruns or provider/local parity evidence reproduced divergent outcomes. | Attempt refs, target/environment refs, and outcome refs proving comparability. |
| `Stable again` | `stable_again` | A prior suspected/reproduced/quarantined/muted state cleared through the required stable evidence window. | Prior verdict/treatment refs plus stable-window attempt refs. |
| `Unknown` | `unknown` | Evidence is missing, contradictory, stale, or not comparable enough to classify. | Denial or missing-evidence reason; no green implication. |
| `Imported only` | `imported_only` | Evidence came from a provider/imported artifact without local parity or comparable replay. | Provider artifact refs and import/comparability posture. |
| `Policy-muted` | `policy_muted` | Admin or enterprise policy mutes delivery or execution for the scoped row. | Policy epoch/rule refs, policy owner, affected scope, and release treatment. |

Transition rules:

| From | To | Gate |
|---|---|---|
| `unknown` | `suspected` | At least one typed observed signal, imported signal, or anchored AI inference lands. |
| `unknown` | `imported_only` | A provider or mirrored artifact is imported without local parity. |
| `imported_only` | `suspected` | Imported evidence contains an intermittency signal, but local or comparable reproduction is absent. |
| `imported_only` | `reproduced` | Local parity or comparable provider reruns reproduce divergent outcomes under matching selector, target, environment, source, and policy context. |
| `suspected` | `reproduced` | Comparable attempts reproduce divergent outcomes and cite the history window. |
| `suspected` | `unknown` | Required evidence is withdrawn, expires, or becomes incomparable. |
| `reproduced` | `policy_muted` | A policy row narrows delivery or execution; the reproduced verdict remains linked as evidence. |
| `suspected` | `policy_muted` | A policy row mutes before reproduction; the suspected evidence remains linked and cannot be promoted to reproduced by policy alone. |
| `policy_muted` | `stable_again` | The policy mute is lifted or expires, and the required stable evidence window lands. Policy removal alone is insufficient. |
| `reproduced` | `stable_again` | Required stable run count/window lands on comparable attempts and prior verdict/treatment refs remain linked. |
| `stable_again` | `suspected` | New intermittent evidence appears after the stable-again closure. |

Rules:

1. AI inference alone may open `suspected` or keep `unknown`; it may not
   move a verdict to `reproduced`.
2. `imported_only` is an evidence-source verdict, not a success state.
   It cannot count as local current truth without parity.
3. `policy_muted` is a policy overlay. It must not erase the underlying
   flaky evidence or release debt.
4. `stable_again` remains visible in the release packet that accepts the
   closure, with prior verdict/treatment refs and stable-window evidence.
5. Any unrecognized verdict or transition becomes `unknown` and blocks
   automatic quarantine, release widening, or green roll-up.

## Snapshot and golden acceptance

The `snapshot_acceptance_review_record` is the governed review sheet for
accepting, rejecting, partially accepting, exporting, or rolling back
snapshot/golden changes. It extends the base execution
`snapshot_review_record` with acceptance granularity, render fallback,
environment scope, and rollback/local-history linkage.

Every snapshot acceptance review MUST publish:

| Field group | Required content | Rule |
|---|---|---|
| Baseline identity | baseline class, baseline artifact refs, proposed artifact refs, baseline digest refs, baseline source refs | Prior and proposed identities must be reviewable without raw snapshot bytes crossing the record. |
| Render mode | rendered structured/pixel/text diff, raw fallback, text fallback, mixed fallback, or unavailable state | A raw fallback is not a normal rendered review; it is a degraded but inspectable mode. |
| Environment scope | serializer/runtime refs, target environment refs, platform/theme/device/data posture, source revision refs | Acceptance applies only to the declared environment scope. |
| Artifact inventory | total artifact count, accepted/rejected/pending/blocked counts, per-artifact refs | Bulk actions still expose file/artifact counts. |
| Granularity | whole review, group, individual artifact, environment slice, or selector slice | A reviewer can tell what was accepted and what remains pending. |
| Decisions | per-artifact accept/reject/block/rollback/export decisions with actor or policy refs | No silent accept-all path. |
| Rollback linkage | local-history refs, VCS refs, managed rollback refs, checkpoint refs, rollback availability | Every applied non-first-capture update has a rollback or local-history path. |

Closed render-mode vocabulary:

- `rendered_structured_diff`
- `rendered_pixel_or_perceptual_diff`
- `rendered_text_diff`
- `mixed_rendered_and_text_fallback`
- `raw_fallback_only`
- `render_unavailable_policy_blocked`
- `render_mode_unknown_requires_review`

Rules:

1. Acceptance requires a pre-apply preview. If no preview exists, the
   review is blocked and the base execution snapshot record uses the
   auto-update-blocked posture.
2. A `raw_fallback_only` or `mixed_rendered_and_text_fallback` review
   must list `fallback_reason_class`, affected artifact refs, and a
   bounded fallback summary. It must not render as a fully structured
   review.
3. `acceptance_granularity_class = whole_review` is allowed only when
   per-artifact counts are present and no artifact remains blocked or
   unreviewable.
4. A partial accept must preserve rejected, pending, blocked, and
   unreviewable counts; accepting one artifact never implies accepting
   the whole baseline group.
5. Any applied non-first-capture update must cite a rollback/local-history
   path. Missing rollback linkage blocks apply.

## AI test-generation gate

The `ai_test_generation_gate_record` is the admission gate for an
AI-generated test proposal. The record exists before any patch lands. It
does not contain raw generated source; it cites patch previews, evidence
packets, selector records, coverage merge results, and sandbox run plans.

Every AI test-generation gate MUST publish:

| Field group | Required content | Rule |
|---|---|---|
| Requested scope | selector refs, canonical test item refs, source object refs, coverage merge refs, test session refs, target environments | Scope is resolved through canonical identity, not display labels. |
| Cited evidence | coverage, bug, diagnostic, review, flaky, source, user-request, or imported-provider evidence refs | A proposal with no cited evidence is blocked. |
| Selector truth | selector resolution class, matched refs, omitted refs, unresolved refs, widening posture | Unresolved or widened selectors block automatic action. |
| Protected-path review | protected path refs, restriction class, owner/reviewer refs, policy refs | Protected-path touches require explicit review and cannot auto-land. |
| Preview requirement | patch preview ref, preview completeness, review sheet ref, rollback/checkpoint refs | Preview is mandatory for every generated test. |
| Sandbox/run plan | sandbox class, planned commands by ref, target environment refs, side-effect posture, result refs when executed | Generated tests must run or remain clearly unexecuted; a plan is not a pass. |
| Landing posture | no-automatic-land class, manual apply gate, side-branch/worktree ref, denial reasons | Claimed stable/protected paths remain proposal-only until reviewed. |

Closed gate-decision vocabulary:

| Value | Meaning |
|---|---|
| `ready_for_user_review` | Evidence, selector truth, preview, and policy checks are sufficient for a human review, but no automatic land is admitted. |
| `preview_required_missing` | Patch preview or review sheet is missing. |
| `blocked_missing_evidence` | The proposal cites no adequate evidence. |
| `blocked_selector_unresolved` | Selector resolution is unresolved, widened without review, or not tied to canonical identities. |
| `blocked_protected_path` | The proposal touches a protected path or claimed stable path without required owner/reviewer approval. |
| `blocked_policy_restriction` | Admin, trust, identity, or policy state blocks proposal or execution. |
| `blocked_sandbox_or_run_required` | The proposal has not executed the required sandbox plan or the sandbox result is incompatible. |
| `gate_decision_unknown_requires_review` | Fail closed. |

Rules:

1. AI-generated tests cannot bypass preview. The patch preview and
   rollback/checkpoint refs must exist before any apply posture is
   offered.
2. AI-generated tests cannot bypass evidence. At least one cited evidence
   ref must support the requested scope, and every cited evidence source
   must retain its freshness/completeness posture.
3. AI-generated tests cannot bypass selector truth. Unresolved selectors,
   display-label-only selectors, or widened selectors block the gate until
   a reviewer accepts the widened scope.
4. AI-generated tests cannot bypass protected-path restrictions. A touch
   to a protected path, claimed stable path, generated source boundary, or
   policy-owned area requires explicit review and cannot auto-land.
5. `no_automatic_land_class` is mandatory. The only conforming values are
   proposal-only or manual-apply-after-review; there is no automatic land
   path for generated tests.
6. A sandbox/run plan may be pending, executed, failed, or blocked. Pending
   or failed plans cannot be summarized as validation success.

## Surface requirements

Every coverage, flaky, snapshot, and AI-test surface MUST keep these
states visible:

- partial coverage merge, missing shard, omitted platform, incompatible
  branch coverage, and stale artifact;
- suspected, reproduced, stable-again, unknown, imported-only, and
  policy-muted flaky verdicts;
- raw fallback, text fallback, unreviewable snapshot artifact, partial
  accept, reject, blocked, rollback unavailable, and environment-scope
  mismatch;
- missing evidence, unresolved selector, preview missing, protected-path
  block, sandbox/run pending, and no-automatic-land posture for
  AI-generated tests.

Surfaces MUST NOT:

- render partial/incompatible coverage as complete;
- use branch coverage zero when branch coverage was unavailable,
  excluded, or incompatible;
- treat imported flaky or coverage evidence as local current truth without
  parity;
- accept snapshot/golden changes without preview and rollback linkage;
- land AI-generated tests automatically, including on paths claimed stable
  or protected by policy.

## Versioning

Each schema declares its own version const at value `1`:

- `coverage_merge_result_schema_version = 1`
- `snapshot_acceptance_review_schema_version = 1`
- `ai_test_generation_gate_schema_version = 1`

Adding a new enum value, optional field, warning class, or fixture is
additive-minor and bumps the relevant version const. Removing or
repurposing an existing enum value is breaking and requires a new
decision row.
