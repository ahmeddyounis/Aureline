# env-inspect beta fixtures

Reviewer fixtures for the env-inspect contract.

Each case fixture pins one seeded scenario (the same scenarios the headless
CLI / inspector binary `aureline_shell_env_inspect` emits) and the expected
core fields the runtime, the shell panel projection, and the headless CLI
all surface. The integration test in
`/crates/aureline-runtime/tests/env_inspect_beta.rs` replays every fixture
through the canonical `EnvInspectSnapshot::from_context` projection and
asserts UI / CLI parity over lane, target class, surface, toolchain class,
boundary cue, trust state, any-degradation flag, review posture, and the
severity bands of every recorded degradation label.

Fixtures:

- `local_terminal.json` — local-host terminal seed, no boundary cue, no
  degradation labels.
- `remote_attach_pending_trust.json` — SSH-remote task seed with pending
  trust evaluation: warning-severity `trust_state_unresolved` label,
  review required.
- `container_devcontainer.json` — devcontainer task seed, boundary cue
  visible, no degradation labels.
- `managed_workspace_restricted.json` — managed-workspace task seed with
  restricted trust state; boundary cue visible, no degradation label
  fires, target-confidence reason set quotes `trust_restricted`.
