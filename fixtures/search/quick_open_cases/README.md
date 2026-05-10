# Quick-open query-session fixtures

Each `*.json` file in this directory describes one quick-open query session:

- the workspace identity and active scope,
- the recent-target / command / lexical inputs,
- the active query and held modifiers,
- the per-source readiness states the chrome must surface, and
- the expected materialized rows in display order.

The fixtures are exercised by
`crates/aureline-shell/tests/quick_open_query_session_tests.rs`. The test
loads each fixture, drives a [`QuickOpenQuerySession`] to the same inputs,
and asserts the materialized snapshot matches `expected_snapshot` exactly.

The fixtures intentionally cover both the protected-walk path (all sources
ready) and the failure drill (lexical lane warming / unavailable while
recents and commands stay usable). They lock in the contract that:

- every row carries its source class and source state explicitly;
- recent targets win on duplicates against lexical rows;
- command rows always quote the canonical `command_id`,
  `disabled_reason_class`, and `invocation_preview_class`; and
- partial-truth causes flow through from the upstream lexical shell.

Renderer-facing entry: `docs/search/quick_open_contract.md`.
