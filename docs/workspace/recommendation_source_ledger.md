# Workspace recommendation-source ledger

This document defines the shared provenance vocabulary Aureline uses
whenever it recommends a **workflow bundle**, **workspace archetype**, or
**readiness task** during project entry. The goal is that entry guidance
is inspectable — not magical — across the shell, Start Center, CLI /
headless output, and support exports.

This contract is normative at the UX and export boundary. Where it
disagrees with the PRD, TAD, TDD, UI/UX specification, or the
workspace-admission / archetype-detection contracts it cites, those
sources win and this document plus its schema and fixtures update in the
same change.

## Companion artifacts

- [`/schemas/workspace/recommendation_source_row.schema.json`](../../schemas/workspace/recommendation_source_row.schema.json)
  - boundary schema for one `recommendation_source_row_record`, used as
    the cross-surface “why did you suggest this?” row.
- [`/artifacts/workspace/archetype_confidence_rows.yaml`](../../artifacts/workspace/archetype_confidence_rows.yaml)
  - defines the stable meaning of `detection_confidence_class` values
    and the evidence-hook vocabulary first-useful-work instrumentation
    can reuse.
- [`/fixtures/workspace/recommendation_cases/`](../../fixtures/workspace/recommendation_cases/)
  - worked YAML cases for the recommendation-source row contract.

This ledger composes with:

- [`/docs/ux/archetype_detection_contract.md`](../ux/archetype_detection_contract.md)
  and [`/schemas/workspace/archetype_detection.schema.json`](../../schemas/workspace/archetype_detection.schema.json)
  for source-labeled detection signals, detected facts, and the
  recommendation / policy / user-choice separation.
- [`/docs/ux/workspace_admission_contract.md`](../ux/workspace_admission_contract.md)
  and [`/schemas/workspace/admission_checkpoint.schema.json`](../../schemas/workspace/admission_checkpoint.schema.json)
  for the post-entry checkpoint and the closed
  `archetype_recommendation_source_class` set.
- [`/docs/ux/start_center_contract.md`](../ux/start_center_contract.md)
  and [`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md)
  for the startup and bundle-card disclosure posture.

## Scope

- Freeze the provenance classes used to explain why a recommendation was
  shown.
- Freeze the cross-surface row shape (`recommendation_source_row_record`)
  that readiness cards, Start Center recommendation rows, bundle
  suggestion chips, CLI/headless projections, and support exports can
  reuse without re-minting vocabulary.
- Reserve an evidence-hook vocabulary that can later be attached to
  first-useful-work instrumentation without changing row shape.

Out of scope: detector implementation, scoring models, recommendation
ranking, auto-apply behavior, and final microcopy.

## Source classes (re-exported)

All recommendation provenance resolves through
`archetype_recommendation_source_class` (frozen in the workspace-admission
contract). A recommendation that cannot resolve at least one source
class MUST NOT reach the user.

| Source class | Meaning | Typical basis |
|---|---|---|
| `detected_facts` | Derived from workspace observations (signals / facts). | Manifest, lockfile, filesystem layout, runtime probe, VCS metadata, workspace files. |
| `heuristic_inference` | Derived from heuristics that are not strong enough to present as a fact or as bundle metadata. | A heuristic rule plus the signals/facts that made it relevant. |
| `bundle_metadata` | Derived from workflow-bundle markers or certified-bundle metadata. | Bundle marker signal or bundle manifest metadata refs. |
| `admin_policy` | Derived from signed policy or fleet policy posture. | Policy signal refs or policy block refs. |
| `prior_user_choice` | Derived from an explicit prior user decision. | Remembered-routing / set-up-later / dismissed-suggestion refs. |
| `extension_contribution` | Derived from installed extensions contributing hints or detectors. | Extension contribution signal refs. |
| `template_default` | Declared by a template or prebuild the user opened from. | Template / prebuild identity refs. |
| `import_packet` | Declared by a portable state, handoff, support, or archive packet. | Import packet signal refs or packet manifest refs. |
| `mixed_recommendation_source` | The row is fed by more than one class above; the row MUST list the contributing classes individually. | A union of the above. |

## Evidence hooks (reserved)

`recommendation_source_row_record.evidence_hooks_reserved[]` reserves a
stable vocabulary first-useful-work instrumentation can later attach to
without changing row shape. Hooks do not replace the source classes; they
supplement them with a machine-readable “what kind of evidence was used”
signal.

Rules:

1. A row MUST carry at least one evidence hook.
2. A row with `recommendation_source_classes[]` that includes
   `heuristic_inference` MUST include `heuristic_inference_applied`.
3. A row whose source includes a template or prebuild MUST include
   `basis_template_or_prebuild`.
4. Hook lists are additive and do not change meaning of the source
   classes.

## Surface projections (normative)

### Readiness cards (post-entry)

- Each readiness card and bucket list MUST allow the user to inspect
  why a suggestion exists by projecting at least one
  `recommendation_source_row_record` per visible suggestion.
- Provenance is guidance, not enforcement: policy blocks and trust
  blocks remain represented by the policy and blocked-task records, not
  by recommendation rows.
- A card that says “Recommended” or “Blocked” without exposing whether
  the underlying guidance came from detected facts, policy, an import
  packet, or a prior explicit user choice is non-conforming.

### Start Center rows and bundle suggestions

- When Start Center surfaces a “recommended” bundle row (including
  imported-handoff suggestions), the row MUST expose recommendation
  provenance using the same source classes and evidence hooks as
  post-entry readiness cards.
- Bundle cards MUST NOT imply a stronger support claim than the
  underlying support/evidence posture allows; the provenance row is
  additive disclosure, not marketing.

### Support / export surfaces

Support exports and docs reproduction packets MUST preserve:

- the row id and schema version;
- the source classes (including `mixed_recommendation_source` rules);
- the basis refs cited by the row; and
- the evidence hook tokens.

Exports MUST NOT embed raw absolute paths, raw URLs with credentials, raw
manifest bodies, raw command lines, raw package-lock contents, or secret
material in row summaries.

