# Stable Runbook Truth Support Artifact

This artifact records the stable support/export packet for runbook source
classes, executable step envelopes, deviation notes, step results, local
checklist ownership, and browser/vendor-console handoff truth.

## Produced Artifacts

- `schemas/support/runbook-step-envelope.schema.json`
- `docs/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth.md`
- `fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/`
- `crates/aureline-support/src/stabilize_runbook_source_step_envelope_and_handoff_truth/mod.rs`

## Stable Claims

- Runbook source classes are typed and attributable.
- Browser-only vendor documentation remains reference-only.
- Mutating in-product steps require the shared action envelope, preview hash,
  approval ref, and audit ref.
- Step execution records preserve `preview_only`, `approved`, `executed`,
  `handoff_required`, and `deviated` states.
- Deviation notes are durable exportable metadata joined to the incident
  timeline.
- Browser and vendor-console pivots carry destination class, reason, return
  anchor, stable handoff refs, and metadata-only export posture.
- Local checklist completion does not mutate provider-owned objects without a
  separately reviewed command.

## Fixture Coverage

- Managed catalog mitigation with action-envelope approval and deviation note.
- Browser-only vendor documentation with explicit vendor-console handoff.
- Local checklist and exported handoff bundle that preserve provider ownership
  without implying provider mutation.
