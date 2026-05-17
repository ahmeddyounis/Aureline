# Shared Detector Beta Fixtures

These fixtures feed the content-integrity beta packet in
`crates/aureline-content-safety::suspicious_content`.

The protected case proves that one detector run produces identical
`bidi_control`, `invisible_formatting`, and `mixed_script_confusable`
warning classes across editor, diff, search, review, docs, safe-preview,
install-review, AI-context, and support-export surfaces. Each surface keeps
raw, rendered, sanitized, and redacted transfer labels explicit and preserves
warning refs into review and support/export flows.
