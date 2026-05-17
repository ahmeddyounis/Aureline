# Doctor probe-pack beta fixtures

Each fixture mirrors the boundary schema at
[`/schemas/support/doctor_probe_pack.schema.json`](../../../../schemas/support/doctor_probe_pack.schema.json).
The corpus exercises every value of the closed `failure_family_class`
vocabulary so beta failure modes have named, read-only probe packs that
declare prerequisites, outputs that map a stable `doctor.finding.*` code
to a recovery-ladder action, and unsupported-state handling:

| Family | Pack fixture | Pack class | Primary recovery routes |
| ------ | ------------ | ---------- | ----------------------- |
| `entry` | `entry_pack.yaml` | `entry_open_readiness` | `locate_missing_target`, `handoff_to_support` |
| `toolchain` | `toolchain_pack.yaml` | `toolchain_resolution` | `reresolve_toolchain`, `open_repair_preview` |
| `search_index` | `search_index_pack.yaml` | `search_index_readiness` | `open_index_status`, `open_repair_preview` |
| `trust_policy` | `trust_policy_pack.yaml` | `trust_policy` | `open_policy_details`, `enter_safe_mode` |
| `git` | `git_pack.yaml` | `git_baseline` | `open_git_baseline_details`, `handoff_to_support` |
| `provider` | `provider_pack.yaml` | `provider_auth` | `reauthenticate_provider`, `handoff_to_support` |
| `restore` | `restore_pack.yaml` | `restore_continuity` | `open_without_restore`, `start_extension_bisect` |

The bundled `catalog.yaml` is the canonical replay set for the seven
families. The protected tests live at
[`crates/aureline-doctor/tests/doctor_probe_packs_beta.rs`](../../../../crates/aureline-doctor/tests/doctor_probe_packs_beta.rs)
and
[`crates/aureline-support/tests/doctor_probe_pack_coverage_beta.rs`](../../../../crates/aureline-support/tests/doctor_probe_pack_coverage_beta.rs).

Every pack preserves the same baseline:

- `schema_ref` is pinned to
  `schemas/support/doctor_probe_pack.schema.json`;
- `doc_ref` is pinned to
  `docs/support/m3/doctor_probe_packs_beta.md`;
- every output declares a `doctor.finding.*` finding code mapped to a
  closed `recovery_ladder_action_class` and a non-empty `recovery_step_ref`;
- `unsupported_state_handling` always names a typed
  `unsupported_state_class`, a distinct `unsupported_finding_code` (never
  reused as an output code), and a governed handoff action.
