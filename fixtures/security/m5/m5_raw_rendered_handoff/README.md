# M5 Raw-versus-Rendered Representation & Handoff Fixtures

These fixtures are valid, export-safe raw-versus-rendered handoff packets that
exercise the projection across the new M5 review, docs, AI, and structured
viewer surfaces. Each one keeps every M5 surface present, exposes the Raw and
Rendered labels and copy/export actions wherever a render transform diverges,
keeps strong-decision surfaces in strict display, and preserves the divergence
warning across all three handoff carriers without leaking raw suspicious bytes.

## byte_identical_no_divergence.json

Every surface uses the `no_transform` render transform, so the rendered form is
byte-identical to the raw bytes. No surface materially diverges: each collapses
to a single canonical-bytes label and a single raw copy action, and the handoff
block preserves no divergence warning. Demonstrates that the projection holds —
and validates — when nothing is rendered differently, so a byte-identical surface
does not silently behave like a diverging one. Regenerate with
`m5_raw_rendered_handoff --clean`.
