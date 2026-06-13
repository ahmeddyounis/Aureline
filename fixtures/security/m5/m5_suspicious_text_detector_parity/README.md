# M5 Shared Suspicious-Text Detector Parity Fixtures

These fixtures are valid, export-safe parity packets that exercise the shared
suspicious-text detector projection across the new M5 surfaces. Each one keeps
every M5 surface present, the same content/threat-class set on every surface,
raw inspection reachable, strong-decision surfaces in strict display, and the
support/admin export free of raw suspicious bytes.

## clean_content_no_warnings.json

Clean source content with no suspicious findings. Every surface is still present
in parity, but with no warnings, no copy choices, and an empty threat-class set.
Demonstrates that the projection holds — and validates — even when the detector
finds nothing, so a clean surface does not silently diverge from a flagged one.
Regenerate with `m5_suspicious_text_parity --clean`.
