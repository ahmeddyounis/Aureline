# Proof packet: install-topology and trust-policy smoke lane

Purpose: anchor proof captures for the unattended trust-policy /
install-topology smoke matrix that proves local-first install
profiles, managed shells, locked credential stores, policy-blocked
actions, side-by-side channels, portable mode, and safe-mode /
restricted preview posture all stay honest on the claimed dogfood
rows.

Reviewer entry point:
[`/artifacts/ops/m1_install_topology_smoke_report.md`](../../ops/m1_install_topology_smoke_report.md).

Canonical sources (non-exhaustive):

- `fixtures/install/m1_topology_rows.yaml` — smoke rows, install-truth
  card claims, expected state-root refs, trust state, credential-store
  posture, denial-reason classes, and failure drills.
- `tests/smoke/trust_policy_install_topology/run_trust_policy_install_topology_smoke.py`
  — unattended runner that replays the matrix and emits the durable
  JSON capture.
- `artifacts/security/trust_state_matrix.yaml` — workspace-trust
  permission-propagation matrix the rows project against.
- `artifacts/release/install_topology_matrix.yaml` — install-profile
  cards the rows must resolve against.
- `artifacts/release/state_root_map.yaml` — durable state-root rows
  with per-channel owning_channels.
- `fixtures/auth/credential_state_cases/store_capability_matrix.yaml`
  — credential-store class and unlock-state vocabulary.
- `artifacts/platform/native_trust_integration_matrix.yaml` —
  per-profile keychain backend labels and trust-store posture.
- `artifacts/platform/claimed_desktop_profiles.yaml` — claimed
  dogfood profile roster the smoke rows must resolve against.
- `artifacts/managed/local_baseline_proof.yaml` — local-baseline
  admissible-surface vocabulary the floor enforcement reuses.
- `artifacts/milestones/m1/dogfood_matrix.yaml` — dogfood rows the
  smoke rows inherit coverage from.

Live runtime consumers (read-only):

- `crates/aureline-shell/src/recovery/safe_mode.rs` — safe-mode
  workspace-restricted contract the safe-mode / restricted-preview row
  projects against.
- `crates/aureline-auth/src/credential_state/mod.rs` — credential-state
  surface the locked-keychain row projects against.
- `crates/aureline-auth/src/browser_callback/mod.rs` — system-browser
  callback contract behind connected-provider open denial.
- `crates/aureline-shell/src/terminal_pane/mod.rs` — terminal-pane
  contract the local-versus-managed shell rows project against.

Validation captures:

- `artifacts/milestones/m1/captures/install_topology_smoke_validation_capture.json`

Refresh: re-run the validation lane after a change to the trust-state
matrix, the install-topology matrix, the state-root map, the
credential-store capability matrix, the native-trust integration
matrix, the local-baseline proof artifact, or the dogfood matrix.

Closure rule: the lane stays open until the latest capture lands under
the governed proof root and every row reports PASS for install
topology, state-root ownership, trust state, denial-reason vocabulary,
local-baseline floor coverage, and the named failure drill.
