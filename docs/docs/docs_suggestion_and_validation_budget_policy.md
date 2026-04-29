# Docs suggestion and validation-budget policy

This document freezes the review-safe docs-suggestion model and the
validation-budget policy used before Aureline may propose README,
changelog, help, symbol-linked docs, stale-example, screenshot,
migration-note, support-window, or benchmark-copy changes.

Machine-readable companions:

- [`/schemas/docs/docs_suggestion.schema.json`](../../schemas/docs/docs_suggestion.schema.json)
  - boundary schema for `docs_suggestion_record`
- [`/artifacts/docs/validation_budget_rows.yaml`](../../artifacts/docs/validation_budget_rows.yaml)
  - validation-budget rows, trigger taxonomy, and no-marketing-lift gates
- [`/fixtures/docs/docs_suggestion_cases/`](../../fixtures/docs/docs_suggestion_cases/)
  - worked cases for broken links, renamed commands, stale screenshots,
    missing migration notes, and unverifiable benchmark copy

Related contracts:

- [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md)
  - citation anchors and symbol-linked reference packets that suggestions
    cite instead of flattening source authority
- [`/docs/docs_integrity/assist_to_help_bridge_contract.md`](../docs_integrity/assist_to_help_bridge_contract.md)
  - provenance retained when assist, docs, help, browser, or support
    surfaces hand off to each other
- [`/docs/docs/docs_pack_manifest_contract.md`](./docs_pack_manifest_contract.md)
  - docs-pack source, version, freshness, and stale-example labels
- [`/artifacts/docs/stale_example_rules.yaml`](../../artifacts/docs/stale_example_rules.yaml)
  - detector registry for examples, guided steps, migration guides,
    screenshots, recipes, and generated examples
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  - freshness windows and rerun triggers for claim-bearing evidence
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  - claim-row posture and channel-propagation rules
- [`/docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  - benchmark-publication evidence and comparability requirements

## Purpose

Docs assistance is allowed to help authors notice drift and draft
reviewable fixes. It is not allowed to invent authority.

Every suggestion therefore carries four things:

1. the target artifact and audience scope;
2. the trigger that caused the suggestion;
3. cited source refs, validation-budget state, and freshness/version
   posture; and
4. an apply posture that states whether the output is draft-only,
   reviewable, locally applicable after validation, publish-handoff-only,
   or blocked pending evidence / owner review.

Suggestions without citations, without a validation-budget row, or with
copy that widens product, support, or benchmark claims are
non-conforming.

## Suggestion record

A `docs_suggestion_record` is the only object a docs-maintenance
surface may render as a proposed README, changelog, help, migration,
symbol-linked reference, stale-example, screenshot, support-window, or
benchmark-copy update.

Required fields are:

- `suggestion_id`, `record_kind`, `docs_suggestion_schema_version`, and
  `created_at`;
- `suggestion_class` and `trigger_class`;
- `target` with artifact kind, path or stable artifact ref, audience,
  branch / release / channel scope, and locale;
- `cited_source_refs`, retaining citation anchors, symbol-linked
  references, claim rows, compatibility rows, benchmark packs, or
  validation outputs by stable ref;
- `validation_budget_state`, pointing at a row in
  `artifacts/docs/validation_budget_rows.yaml`;
- `freshness_state` and, where applicable, `version_match_state`;
- `stale_detection_state`, using the closed states below; and
- `apply_posture`, `proposed_change`, `owner_review`, and
  `no_marketing_lift_gate`.

The record may contain draft prose or a diff ref, but that prose is
never authority. Authority remains in the cited sources.

## Suggestion classes

| Suggestion class | Typical target | Required posture |
|---|---|---|
| `readme_update` | README or module docs | Review diff only until citations and validation budget are current. |
| `changelog_update` | changelog or release notes | Release/channel scope required; claim-bearing copy points to a claim row or reviewed pack. |
| `help_update` | Help/About, docs pane, docs browser, service-health help row | Must preserve citation anchors and source/version/freshness chips. |
| `stale_example_finding` | code block, command transcript, recipe, screenshot, generated example | Must carry a stale-detection state and validation mode. |
| `symbol_linked_doc_suggestion` | generated reference, symbol card, hover/docs link | Must carry a symbol-linked-reference ref or a repair hook. |
| `migration_note_prompt` | migration notes, upgrade guide, release notes | Prompt-only until the compatibility row, claim row, or migration packet is cited. |
| `support_window_alignment` | support matrix, compatibility table, CLI/help support text | Owner review required whenever support level or support window changes. |
| `benchmark_copy_review` | benchmark copy, public-proof packet, README performance claim | Blocked until benchmark-publication evidence is current and comparable. |
| `screenshot_refresh` | screenshot or visual walkthrough asset | Draft refresh only until the screenshot anchor and rendered surface are revalidated. |

## Stale-detection states

Stale-example and docs-drift findings use exactly three detection
states when the state is applicable:

| State | Meaning | Apply posture |
|---|---|---|
| `proven_broken` | A validator or resolver observed a concrete failure, such as a 404, missing anchor, renamed command, failed snippet, or unsupported route. | A narrowing or repair diff may be reviewed; publication still follows the validation budget. |
| `suspected_stale` | Static signals or anchor drift indicate the example is likely out of date, but the failing behavior has not been reproduced. | Draft/review only; no automatic apply. |
| `unchanged_unverified` | The source has not changed, but required validation is missing, expired, unsupported in the current environment, or outside budget. | No content rewrite unless it narrows or discloses the unverified state. |

Rows that are not stale-example findings set `stale_detection_state =
not_applicable`.

## Trigger taxonomy

Every suggestion uses one `trigger_class` from the schema. The initial
taxonomy is:

- `broken_link` - target, anchor, destination descriptor, or allowed
  host no longer resolves.
- `renamed_command` - command descriptor, alias, palette row, CLI help,
  or transcript references a renamed command.
- `renamed_setting` - settings vocabulary changed and docs still quote
  the old key or default.
- `renamed_symbol` - symbol-linked reference or generated reference
  points at a renamed / moved symbol.
- `stale_screenshot` - screenshot anchor, route, surface chip, or
  visual state no longer matches the current destination descriptor.
- `missing_migration_note` - schema, command, config, compatibility, or
  support-window change needs a migration note before release-facing
  docs can widen or preserve a claim.
- `support_window_mismatch` - docs/support copy disagrees with a
  compatibility row, support-window row, or release-family rule.
- `benchmark_reference_unverifiable` - benchmark or performance copy
  lacks a current, comparable benchmark-publication packet.
- `api_contract_changed`, `command_output_drift`,
  `docs_pack_freshness_expired`, `version_mismatch`,
  `claim_row_changed`, `failing_snippet`, and `stale_example` cover the
  remaining existing docs-integrity signals.

Adding a trigger is additive when it has a validation-budget row and a
fixture. Reusing a trigger for a different meaning is breaking.

## Validation budgets

Validation budgets define what must be revalidated before a suggestion
surface may propose or apply a change. They are not freshness hints;
they are gates.

The seed rows cover:

- examples;
- commands and CLI transcripts;
- snippets;
- screenshots;
- support matrices / support-window copy; and
- benchmark references.

Each row names:

- the artifact kinds it covers;
- required validation modes;
- the maximum validation age or event-based expiry;
- the trigger classes that force immediate revalidation;
- the surfaces blocked when the budget is expired; and
- the maximum apply posture allowed while expired.

When a budget is expired, missing, or unsupported in the current
environment, suggestion surfaces may still open a review packet, but
they may not present the draft as ready to publish or apply.

## No-marketing-lift rule

Suggestions may narrow, correct, or disclose. They may not create new
claims.

A suggestion is blocked pending owner review when it would:

- introduce a new product capability claim;
- widen a support level, support window, compatibility statement, or
  migration guarantee;
- make a benchmark, performance, efficiency, or comparison statement
  without a current benchmark-publication packet;
- remove a known limit or exclusion without claim-row evidence; or
- phrase a draft in a way that implies a stronger release, support, or
  benchmark posture than the cited source permits.

The schema records this through `no_marketing_lift_gate` and
`proposed_change.claim_change_posture`. The allowed default is
`no_claim_change`. Any claim widening is represented as a blocked
posture, not as an apply-ready suggestion.

## Publication boundary

Docs suggestions are local review artifacts until a reviewer or publish
handoff explicitly accepts them. A rendered markdown preview, generated
diff, screenshot refresh, AI-written paragraph, or migrated snippet is
not proof of correctness.

Publication handoff packets must retain:

- suggestion id;
- target artifact and release/channel scope;
- cited source refs and citation anchors;
- validation-budget row and current state;
- draft/apply posture;
- owner-review state; and
- any no-marketing-lift blockers.

Browser or provider handoffs reuse the existing handoff packets; raw
URLs do not enter `docs_suggestion_record`.

## Fixture coverage

The seed fixture set covers:

- a proven broken link;
- a renamed command;
- a suspected stale screenshot;
- a missing migration note; and
- unverifiable benchmark copy.

These cases intentionally include blocked and draft-only postures so
implementations exercise the denial paths as well as happy-path review
diffs.
