# Onboarding Task-Success Telemetry Fixtures

This directory pins fixture expectations for the onboarding telemetry runtime.
The Rust tests generate the canonical event capture through
`aureline_telemetry::onboarding::seeded_design_partner_capture`, then compare
the generated flow coverage, first-useful-work timing, migration funnel counts,
and privacy posture against these fixture rows.

The fixture is intentionally metadata-only. It must not contain raw project
content, repository names, file paths, URLs, prompts, terminal text, clipboard
content, or credentials.
