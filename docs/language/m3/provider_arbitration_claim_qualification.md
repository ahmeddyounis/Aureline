# Multi-provider language-arbitration claim qualification

This document freezes the release-bearing proof contract that protects
beta marketing claims on Aureline's language-action lanes. It tells
release, support, partner-review, and shiproom audiences which arbitration
evidence must be current before a claimed beta language row may carry
daily-driver semantics in editor chrome, quick-fix preview, diagnostics
detail, command result, CLI/headless inspect, or support export.

This contract composes with and does not replace the inspector beta
contract frozen in
[`/docs/language/m3/provider_arbitration_beta.md`](provider_arbitration_beta.md);
that document binds the per-provider health row and the per-lane
arbitration decision shape. This document binds the corpus, the proof
report, and the downgraded semantic-claims matrix that the marketing
gate reads. If this document disagrees with the PRD, TAD, TDD, or the
inspector beta contract, those documents win and this document plus its
companion artifacts update in the same change.

Machine-readable companions:

- [`/fixtures/language/m3/provider_arbitration_corpus/`](../../../fixtures/language/m3/provider_arbitration_corpus/)
  — the checked-in proof corpus. Every claimed beta language row points
  back into this directory before the row may ship.
- [`/artifacts/language/m3/provider_arbitration_report.md`](../../../artifacts/language/m3/provider_arbitration_report.md)
  — the human-readable red/green release-evidence report.
- [`/artifacts/language/m3/downgraded_semantic_claims_matrix.json`](../../../artifacts/language/m3/downgraded_semantic_claims_matrix.json)
  — the structured matrix support, partner reviews, and beta claim
  manifest renderers read.
- [`/schemas/language/provider_health_state.schema.json`](../../../schemas/language/provider_health_state.schema.json)
  and
  [`/schemas/language/arbitration_decision.schema.json`](../../../schemas/language/arbitration_decision.schema.json)
  — the inspector's frozen boundary shapes that every corpus entry
  satisfies bit-for-bit.

## Why this contract exists

Language intelligence on real beta workspaces fans into many providers
at once. The arbitration inspector beta contract already prevents a
single provider from silently winning when conflict, partial scope, or
unhealthy state would otherwise be hidden. This document goes one step
further: it turns the inspector from a helpful surface into a release
gate. A claimed beta row may not market a lane as semantic unless the
corpus contains current evidence for every scenario class below.

Without this gate it is possible to:

- claim "exact rename" while the corpus only proves exact rename for a
  single-file scope and never exercises the wide-scope partial path;
- claim "authoritative formatting" while the corpus has no labeled
  text-fallback drill against a degraded language service;
- claim "definition coverage" while the only crash-loop drill in the
  corpus is for an old language service and never for the framework
  pack or notebook adapter that the marketed row actually depends on;
- ship a "user preference reorder" feature that hides conflict or stale
  warnings simply because the higher-ranked provider answered first.

The proof corpus closes those silent-regression paths. Every claimed
beta language row that markets a lane must point at one of the corpus
entries below and may not ship if any required scenario class is empty.

## Required scenario classes per lane

Each claimed beta language row that markets a lane must show current
proof for the following scenario classes:

| Scenario class | Required evidence |
|---|---|
| `provider_agreement` | At least one corpus entry per lane where every admissible provider agrees and the apply gate is `ready_to_apply`. |
| `provider_disagreement` | At least one corpus entry per claimed beta row where providers conflict and the apply gate is not `ready_to_apply`; the disagreement visibility must not be `none`. |
| `partial_scope` | At least one corpus entry per claimed beta row where negotiated completeness is `partial_for_claimed_scope` and apply routes through `preview_required`, `side_branch_required`, `blocked_for_partial_scope`, or `inspect_only`. |
| `imported_snapshot` | At least one corpus entry where the winning provider's locality is `imported_snapshot` and the narrowed-scope downgrade is disclosed. |
| `stale_cache_reuse` | At least one corpus entry where the outcome is `stale` and the fallback label is `cached_semantic_reuse`. |
| `provider_crash_loop` | One corpus entry per affected provider family (language-server, framework-pack, notebook-adapter) where the provider is `crash_loop_quarantined`, apply is `blocked_for_health`, retry/isolate actions are exposed, and `quarantine_ref` is set. |
| `wide_scope_rename` | At least one corpus entry per language row that markets rename where the rename is wide-scope, negotiated completeness is partial, and the apply gate is `preview_required` or `side_branch_required`. |
| `text_fallback` | At least one corpus entry per language row that markets formatting or rename where the labeled fallback class is `text_fallback` and the downgraded-promise reason is `fallback_to_text`. |
| `provider_preference_reorder` | At least one corpus entry where user preference re-orders the provider order yet the inspector keeps conflict visible, and one where stale warnings remain visible despite user-preferred ranking. |

## Claim-status classification

Every corpus row is reduced into one of three claim-status classes:

- `qualified_for_beta_claim` — outcome is `exact`, conflict is `none`,
  apply gate is `ready_to_apply`. The lane may carry a marketed
  semantic claim for the represented scope.
- `downgraded_disclose_and_proceed` — outcome is non-exact OR conflict
  is non-empty OR apply gate enforces preview / side-branch review. The
  lane may proceed but the downgrade must be disclosed on every consumer
  surface using the same downgraded-promise copy, fallback label, and
  apply gate that the corpus row freezes.
- `blocked_for_recovery` — outcome is `unavailable`. Apply is blocked
  for health and the row cannot carry the marketed claim until recovery
  succeeds.

The downgraded semantic-claims matrix at
`/artifacts/language/m3/downgraded_semantic_claims_matrix.json` carries
one row per corpus entry with the lane, scenario class, outcome, apply
gate, fallback label, downgrade reason, conflict class, fixture ref, and
claim-status class. Beta claim manifests, partner-review packets, and
support bundles all read this matrix to render the same red/green view.

## Cross-surface routing

Every corpus entry routes the arbitration decision to the same six
consumer surfaces: `editor_chrome`, `quick_fix_preview`,
`diagnostics_detail`, `command_result`, `cli_headless_inspect`, and
`support_export`. The inspector contract enforces that each surface
reads the same record; this proof contract enforces that the record is
the only source of truth for marketed claims.

Apply gates must remain consistent across surfaces:

- `ready_to_apply` is admissible only for `exact` outcomes with empty
  conflict;
- `preview_required` and `side_branch_required` are admissible for
  non-exact outcomes with a visible disagreement or partial scope;
- `blocked_for_health`, `blocked_for_partial_scope`, and
  `blocked_for_disagreement` are admissible only when the outcome would
  otherwise be `unavailable` or the conflict is unresolvable.

## Redaction posture

Corpus entries, the downgraded semantic-claims matrix, and the
arbitration proof report all use the `metadata_safe_default` redaction
class. They carry opaque provider, host, epoch, workset, workspace,
artifact, symbol, cell, command, policy, and execution-context handles
plus typed vocabulary and reviewable summaries. They do not carry raw
source text, raw notebook bodies, raw generated artifact payloads, raw
provider logs, raw hostnames, raw URLs, raw process arguments, or raw
secret material.

Support bundles, partner reviews, and beta claim manifests may include
the matrix and the proof report verbatim without further redaction.

## Rotation and review

This contract and the proof corpus are rotated together. Any change to a
claimed beta row's lane-support matrix, provider stack, or fallback
contract must rotate at least one fixture and the matrix that references
it. The contract document and the matrix update in the same change so
release, support, and partner reviews always see the same source of
truth.

## Acceptance criteria

This contract holds when:

- every claimed beta language row points at corpus fixtures covering
  every required scenario class for its marketed lanes;
- the inspector validator passes on the corpus with zero defects;
- the downgraded semantic-claims matrix lists one row per fixture and
  classifies every row into a known scenario class and claim-status
  class;
- no `ready_to_apply` row carries a non-empty conflict, fallback label,
  or downgrade reason;
- every `unavailable` row blocks apply and exposes retry/isolate
  controls plus recovery hints;
- crash-loop drills exist for the language-server, framework-pack, and
  notebook-adapter families that the marketed beta rows depend on;
- the surface routing covers all six consumer surfaces on every fixture.

When any of these invariants is broken, the corpus replay test fails,
the matrix builder fails, and beta marketing claims for the affected
row may not republish until the corpus and matrix are repaired.
