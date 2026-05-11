# Install-topology and trust-policy smoke report

Reviewer-facing entry point for the unattended trust-policy /
install-topology smoke lane that proves local-first install profiles,
managed shells, locked credential stores, policy-blocked actions,
side-by-side channels, portable mode, and safe-mode / restricted
preview posture all stay honest on the claimed dogfood rows.

The lane is a closed loop:

1. **Matrix** —
   [`fixtures/install/m1_topology_rows.yaml`](../../fixtures/install/m1_topology_rows.yaml)
   names the smoke rows, the dogfood row each one inherits coverage
   from, the install-profile card it claims, the expected state-root
   refs, the trust state, the credential-store posture, the
   shell mode, and the safe-preview posture.
2. **Runner** —
   [`tests/smoke/trust_policy_install_topology/run_trust_policy_install_topology_smoke.py`](../../tests/smoke/trust_policy_install_topology/run_trust_policy_install_topology_smoke.py)
   replays every row against the canonical sources and emits a durable
   JSON capture.
3. **Capture** —
   [`artifacts/milestones/m1/captures/install_topology_smoke_validation_capture.json`](../milestones/m1/captures/install_topology_smoke_validation_capture.json)
   records pass/fail per row plus per-row diagnostics and (when
   `--force-drill` is used) the forced-drill replay record.
4. **Proof packet** —
   [`artifacts/milestones/m1/proof_packets/install_topology_smoke.md`](../milestones/m1/proof_packets/install_topology_smoke.md)
   anchors the lane in the proof index.

## Run the lane

```bash
python3 tests/smoke/trust_policy_install_topology/run_trust_policy_install_topology_smoke.py \
    --repo-root .
```

Exit code is non-zero if any row fails the protected walk (install
topology, state-root ownership, trust state, denial-reason class,
local-baseline floor, claimed-profile resolution, or required
disclosures) or the failure drill stops being reproducible.

## Smoke rows seeded

| Smoke row | Inherits dogfood row | Install card | Dimension covered |
| --- | --- | --- | --- |
| `aureline.install.smoke.local_first_no_account_per_user_stable_macos` | `dogfood.fixture_repo.plain_text_notes` | `card.macos.universal_binary.per_user_installed.stable` | local-first no-account entry |
| `aureline.install.smoke.managed_signed_in_per_machine_stable_windows` | `dogfood.fixture_repo.nested_source_tree` | `card.windows.x86_64.managed_deployed.stable` | managed / local-versus-managed shell vocabulary |
| `aureline.install.smoke.locked_keychain_local_first_linux` | `dogfood.fixture_repo.path_and_encoding` | `card.linux.x86_64.per_user_installed.stable` | locked or unavailable credential store |
| `aureline.install.smoke.policy_blocked_managed_egress_macos` | `dogfood.fixture_repo.missing_target_restore` | `card.macos.universal_binary.managed_deployed.stable` | policy-blocked action |
| `aureline.install.smoke.side_by_side_preview_stable_windows` | `dogfood.fixture_repo.plain_text_notes` | `card.windows.x86_64.side_by_side_preview.preview` | side-by-side channels |
| `aureline.install.smoke.portable_no_host_mutation_linux` | `dogfood.fixture_repo.nested_source_tree` | `card.linux.x86_64.portable.portable_stable` | portable no-host mutation |
| `aureline.install.smoke.safe_mode_restricted_preview_macos` | `dogfood.fixture_repo.missing_target_restore` | `card.macos.universal_binary.per_user_installed.stable` | safe-mode / restricted preview |

The protected walk for each row asserts that install topology and
state-root ownership match the install-truth card, the trust state is
in vocabulary, the credential-store posture stays honest, the
local-baseline floor is reachable, denials are typed and actionable,
and the row's named failure drill reproduces a precise `check_id`.

## Failure drill posture

Every row names a forced-failure input plus the precise `check_id` the
matrix records when the row drifts away from canonical truth. Run the
drill against the runner directly:

```bash
python3 tests/smoke/trust_policy_install_topology/run_trust_policy_install_topology_smoke.py \
    --repo-root . \
    --force-drill <smoke_row_id>:<drill_id>
```

The runner exits `0` only when the drill reproduces the row's declared
`expected_check_id`; otherwise it exits non-zero so the lane cannot
silently regress.

## Where this fits in the claimed dogfood loop

- **Dogfood rows** are reused from
  [`artifacts/milestones/m1/dogfood_matrix.yaml`](../milestones/m1/dogfood_matrix.yaml);
  every smoke row's `inherited_dogfood_row_id` must resolve there.
- **Trust state** is reused from
  [`artifacts/security/trust_state_matrix.yaml`](../security/trust_state_matrix.yaml);
  the smoke runner refuses unknown vocabulary.
- **Install topology and state roots** are reused from
  [`artifacts/release/install_topology_matrix.yaml`](../release/install_topology_matrix.yaml)
  and
  [`artifacts/release/state_root_map.yaml`](../release/state_root_map.yaml).
- **Credential-store classes** are reused from
  [`fixtures/auth/credential_state_cases/store_capability_matrix.yaml`](../../fixtures/auth/credential_state_cases/store_capability_matrix.yaml).
- **Claimed dogfood profiles** are reused from
  [`artifacts/platform/claimed_desktop_profiles.yaml`](../platform/claimed_desktop_profiles.yaml).
- **Local-baseline floor** is reused from
  [`artifacts/managed/local_baseline_proof.yaml`](../managed/local_baseline_proof.yaml).

## Refresh policy

Refresh the capture and update `as_of` when any of the following
change:

- the trust-state matrix
  (`artifacts/security/trust_state_matrix.yaml`);
- the install-topology matrix
  (`artifacts/release/install_topology_matrix.yaml`);
- the state-root map
  (`artifacts/release/state_root_map.yaml`);
- the credential-store capability matrix
  (`fixtures/auth/credential_state_cases/store_capability_matrix.yaml`);
- the native-trust integration matrix
  (`artifacts/platform/native_trust_integration_matrix.yaml`);
- the local-baseline proof artifact
  (`artifacts/managed/local_baseline_proof.yaml`); or
- the dogfood matrix
  (`artifacts/milestones/m1/dogfood_matrix.yaml`).

Stale captures are surfaced by
[`ci/check_m1_artifact_index.py`](../../ci/check_m1_artifact_index.py).
