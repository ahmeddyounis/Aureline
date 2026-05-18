# Large-file mode and save-participant risk

This contract binds the editor large-file surface to staged-save risk review.
The goal is that oversized, binary-like, generated, hostile, or
round-trip-risky files remain inspectable without letting formatters,
organize-imports, code actions, or AI apply flows quietly widen the write
scope.

## Limited-mode files

Files enter limited mode through the existing classifier: size threshold,
resource pressure, content classification, decode posture, or explicit user
choice. The editor projects that decision as
`limited_mode_file_record` using
`schemas/editor/limited_mode_file.schema.json`.

The record must include:

- the activation trigger and classifier reason;
- safe preview posture;
- reduced edit and write policy;
- capability rows for denied, downgraded, and allowed lanes;
- the explicit `Open anyway` override disclosure; and
- support-safe summary fields with raw payload excluded.

Limited mode does not claim normal editor parity. Full-file syntax parsing,
background indexing, full-file diagnostics, whole-file save participants, and
whole-file AI apply are disabled or downgraded before a lane starts work.

## Save-participant risk

The staged save coordinator emits a `save_participant_risk_review_record`
using `schemas/editor/save_participant_risk.schema.json`. Participants declare
their class, output origin, expected file effects, rewrite class, review
triggers, and checkpoint posture before they run on staged content.

The coordinator blocks before staged mutation when a participant declares:

- whole-file rewrite or whole-file fallback;
- generated/protected or multi-file effects;
- AI output without a reviewed ticket;
- policy or trust blocking; or
- unknown scope, origin, or safety.

Participants that look safe but produce a whole-file rewrite are stopped before
durable write. If on-disk identity changes after participants run, the save
returns a rebase/review outcome instead of overwriting. If participant output
would change line-ending or final-newline posture, durable write is held for
review so source-fidelity conversion is explicit.

## Fixtures

The fixture set under `fixtures/editor/m3/large_file_and_whole_rewrite/`
covers:

- binary-like limited-mode safe preview;
- whole-file formatter rewrite blocked before staged mutation; and
- source-fidelity review after staged participant output changes EOL or final
  newline posture.

These records are metadata-only and support-export safe by construction.
