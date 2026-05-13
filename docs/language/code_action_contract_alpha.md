# Code-action alpha runtime contract

This document describes the first checked-in runtime lane for quick fixes,
fix-all actions, source actions, generated synchronization, and read-only
validation actions. The implementation lives in
`aureline-language::code_actions` and consumes the diagnostic bus provenance
records instead of inventing a second source or freshness model.

The existing diagnostic convergence contract remains the vocabulary source for
diagnostic clustering, freshness, semantic-layer state, preview requirements,
and suppression or baseline review. This alpha lane adds the runtime admission
rules that decide whether an action can apply inline or must open review first.

## Runtime records

`CodeActionRecord` carries:

- acting provider identity, source kind, support class, freshness, locality,
  semantic-layer state, and epoch bindings;
- action class and plain-language label;
- linked diagnostic refs;
- side-effect class, safety class, mutation scope, preview requirement, and
  apply posture;
- generated, protected, blocked, configuration, dependency, file, anchor, and
  diagnostic counts;
- validation and replay hints;
- content-integrity cues such as suspicious-text finding refs; and
- a named undo group for every mutating action that claims an apply path.

`CodeActionAdmissionRecord` is the first consumer-facing admission packet. It
answers whether preview is required, whether silent apply is admitted, which
side-effect class is involved, and why direct apply was refused.

`CodeActionAlphaSnapshot` and `CodeActionSurfaceProjection` expose the same
action records to editor action pickers, review previews, CLI JSON, and support
exports without reshaping the safety contract per surface.

## Side-effect classes

The runtime side-effect classes are:

| Class | Meaning |
|---|---|
| `no_mutation_validation_only` | Refreshes or validates evidence without writing state. |
| `current_anchor_text_edit` | Edits only the current diagnostic anchor. |
| `current_file_text_edit` | Edits the current file beyond the exact anchor. |
| `whole_file_rewrite` | Rewrites the current document. |
| `multi_file_workspace_edit` | Changes more than one file or workspace object. |
| `configuration_or_dependency_mutation` | Changes configuration, dependency, policy, or repo truth. |
| `generated_or_protected_mutation` | Touches generated or protected paths. |
| `unknown_provider_mutation` | Provider did not disclose a stable mutation boundary. |

Only current-anchor and current-file edits can be admitted for inline apply,
and only when all other admission fields also allow it.

## Preview and apply rules

The runtime refuses silent apply when any of these are true:

- preview requirement is structured diff, batch scope, generated/protected
  preview, or policy/repo mutation preview;
- side-effect class is whole-document, multi-file, configuration/dependency,
  generated/protected, or unknown;
- mutation scope is whole-document, multi-file, generated, or policy-scoped;
- mutation counts show multiple files, generated paths, protected paths,
  blocked paths, configuration mutation, or dependency mutation;
- provider freshness or semantic layer is below the current safe floor;
- content-integrity cues require safe preview; or
- a mutating action lacks a named, attributable undo group.

This keeps quick fixes reviewable under degraded conditions while allowing a
small current-anchor quick fix to apply inline when the provider is current and
the undo group is named.

## Undo groups

Every mutating action must provide:

- a stable undo group id;
- a user-visible group label;
- the command id that applies the mutation;
- the provider or actor ref that owns the group;
- a reversal class; and
- a checkpoint ref when reversal depends on one.

Read-only validation actions do not need undo metadata and must use the
read-only apply posture.

## Protected fixture

The fixture at
`fixtures/language/code_action_alpha/action_cases.json` covers:

- a single-anchor quick fix admitted for inline apply;
- a multi-file fix-all action forced through preview;
- a configuration/dependency mutation blocked from silent apply;
- a generated/protected sync action with content-integrity preview cues; and
- a read-only validation recheck with no undo group.

The Rust test `crates/aureline-language/tests/code_action_alpha.rs` builds the
provider descriptors from diagnostic source descriptors, publishes the action
records to the catalog, checks admission decisions, verifies aggregate counts,
and round-trips the export-safe snapshot through JSON.
