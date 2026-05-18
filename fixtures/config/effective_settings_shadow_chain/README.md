# Effective Settings Shadow Chain Fixtures

These fixtures exercise the shared settings resolver path used by desktop, CLI, sync, policy, and support export projections.

- `policy_locked_effective_record.json` is emitted by `aureline_settings_inspect effective-record security.ai.egress_policy`.
- `scope_explicit_write_preview.json` is emitted by `aureline_settings_inspect preview-write` and trimmed to the stable fields needed for scope-preview review.

The case proves that admin policy remains visible as a ceiling, user values remain visible when capped, and a write preview names the exact target scope and artifact instead of widening silently.
