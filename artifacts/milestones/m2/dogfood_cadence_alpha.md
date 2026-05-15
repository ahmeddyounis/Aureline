# Daily-Driver Dogfood Cadence (Alpha)

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T19:01:57Z
stale_after: P14D
source_revision: git:ea10b7a3
trigger_revision: alpha_dogfood_cadence_contract_set@2026-05-15
validation_capture: artifacts/milestones/m2/captures/dogfood_cadence_validation_capture.json
claim_change_state: no_claim_widening
audience: internal_dogfood_drivers
publication_posture: internal_only_no_partner_widening
```

Purpose: make the M1→M2 entry-gate condition "daily dogfooding works on small
projects" citable from a single in-tree artifact. The substrate is real
(the `aureline_shell` binary integrates the daily-driver crates per
`crates/aureline-shell/src/bin/aureline_shell.rs` →
`aureline_shell::bootstrap::run_native_shell`), but cadence and rotation
evidence was not previously visible in-tree. This packet records that
cadence against the protected reference-workspace seeds and the headless
edit/save route, and ties each session to a launch bundle and wedge.

## Packet Header

| Field | Value |
|---|---|
| Packet id | `dogfood_cadence.external_alpha.first` |
| Packet state | `seeded_with_known_limits` |
| Entry-gate clause | `M1->M2 §8.2 — daily dogfooding works on small projects` |
| Substrate ref | `crates/aureline-shell/src/bin/aureline_shell.rs` |
| Bootstrap ref | `aureline_shell::bootstrap::run_native_shell` |
| Baseline build health | `artifacts/milestones/m2/baseline_build_health.md` |
| Onboarding telemetry contract | `crates/aureline-telemetry/src/onboarding/mod.rs` |
| Latest capture | `artifacts/milestones/m2/captures/dogfood_cadence_validation_capture.json` |

## Canonical Inputs

- Entry-gate phrasing: `.t2/docs/Aureline_Milestones_Document.md` §8.2
- Internal dogfood guide (M1 lane, still authoritative for cadence loop):
  `docs/dogfood/m1_internal_dogfood.md`
- Small-project matrix (rotation source): `artifacts/milestones/m1/dogfood_matrix.yaml`
- Feedback intake taxonomy: `artifacts/dogfood/feedback_taxonomy.yaml`
- Known-limits packet: `artifacts/milestones/m2/known_limits_alpha.yaml`
- Reference-workspace seeds: `fixtures/workspaces/reference/`
- Launch bundles: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`,
  `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Build identity: `artifacts/build/build_identity.json`
- Onboarding telemetry: `crates/aureline-telemetry/src/onboarding/`

## Privacy Posture

This packet records cadence against protected, synthetic, in-tree fixtures
only. Per the dogfood guide and the external-alpha known-limits packet:

- No raw partner content is included.
- No real user filesystem paths are referenced — every workspace path
  resolves under `fixtures/workspaces/reference/` (protected seeds with
  `visibility_class: public`, `license_status: synthetic_no_real_content`).
- The headless-test working tree used by the saved-byte session is a
  `mktemp -d` scratch path that does not name any private project; only
  the fixture identity (`refws.*`) is referenced.

## Rotation of Small Projects

The daily-driver rotation cycles four protected reference workspaces. Each
row resolves to an in-tree fixture, so the rotation can be replayed by a
clean clone of the repository.

| Row | Reference workspace id | Fixture path | Archetype hint | Size class |
|---|---|---|---|---|
| `rotation:ts_web_app` | `refws.ts_web_app_archetype_seed` | `fixtures/workspaces/reference/ts_web_app_archetype_seed.json` | `ts_web_app` | `tiny` |
| `rotation:python_data_app` | `refws.python_data_app_archetype_seed` | `fixtures/workspaces/reference/python_data_app_archetype_seed.json` | `python_data_app` | `tiny` |
| `rotation:micro_local_folder` | `refws.micro_local_folder` | `fixtures/workspaces/reference/micro_local_folder.json` | `misc_folder` | `micro` |
| `rotation:partially_ready_restore` | `refws.partially_ready_restore_seed` | `fixtures/workspaces/reference/partially_ready_restore_seed.json` | `misc_folder` | `tiny` |

The M1 small-project matrix at `artifacts/milestones/m1/dogfood_matrix.yaml`
remains the authoritative scenario source for the per-row daily action set
(open → quick open → edit/save → terminal → restore → missing-target
recovery). This packet extends that rotation to the M2 reference-workspace
identities so the cadence stays addressable from the M2 scoreboard.

## Sessions

Each session below is reproducible from the named build identity, the
named fixture, and the headless route exposed by the daily-driver binary.
A "full session" exercises at least one action from each required
category in the small-project matrix (open, quick open, edit_save,
terminal, restore_session, missing_target_recovery).

### Session 1 — Headless edit/save sanity (most recent full session)

| Field | Value |
|---|---|
| Session id | `dogfood_session:external_alpha.headless_edit_save.2026-05-14` |
| Date of session | `2026-05-14` |
| Exact build identity | `build-id:aureline:dev:0.0.0:aarch64-apple-darwin:dev:05649f1324e2` |
| Build identity source | `artifacts/milestones/m2/baseline_build_health.md` |
| Bundle exercised | `launch_bundle:typescript_web_app.seed` (analogue — fixture is a plain-text scratch shape) |
| Wedge exercised | `alpha_wedge:typescript_javascript` (binary entry path; wedge-bound proofs remain blocked) |
| Reference workspace | `refws.partially_ready_restore_seed` (synthetic plain-text scratch tree) |
| Workspace synthesis | `mktemp -d` with a single `notes.txt` (`old\n`) — no private filesystem path |
| Route | `--open <synth> --headless-test-edit-save notes.txt --headless-test-write-hex <hex> --headless-test-report <path>` |
| Actions exercised | open, edit_save, save-token / atomic_replace journaling |
| Outcome | `pass` (exit 0; report `outcome: committed`, `write_strategy: atomic_replace`) |
| Saved-byte SHA-256 | `536210a10cc37f83ce51871b7ea19ad2243988c2bc7ec8758a93bdd3c8c814ae` |
| Known issues hit | none on the exercised path; degraded-state copy of windowed startup not exercised (see Known Limits below) |

Reproduction:

```sh
tmp=$(mktemp -d)
printf 'old\n' > "$tmp/notes.txt"
cargo run -p aureline-shell --bin aureline_shell -- \
  --open "$tmp" \
  --headless-test-edit-save notes.txt \
  --headless-test-write-hex 68656c6c6f2d66726f6d2d686561646c6573730a \
  --headless-test-report "$tmp/report.json"
```

### Session 2 — Workspace strict clippy + workspace check (toolchain health)

| Field | Value |
|---|---|
| Session id | `dogfood_session:external_alpha.toolchain_health.2026-05-15` |
| Date of session | `2026-05-15` |
| Exact build identity | `artifacts/build/build_identity.json` (commit `b7ee32adb5eb`) |
| Bundle exercised | n/a (toolchain-level only) |
| Wedge exercised | substrate (no wedge-specific surface exercised) |
| Reference workspace | n/a (in-tree workspace) |
| Actions exercised | `cargo fmt --all --check`, `cargo check --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `git diff --check` |
| Outcome | `pass` (all four commands exit 0; no warnings under strict clippy) |
| Known issues hit | none |

This session is recorded so the cadence log is honest about what was
*actually* exercised between scoreboard refreshes; it is not a daily-loop
session against a small project, but it is a daily-driver health check
and is therefore listed.

## Sessions Not Yet Exercised In-Tree

The four full daily-loop sessions below are reserved rows. They are
expected to land via the corresponding scoreboard rows; this packet does
not claim them as completed.

| Reserved session id | Reference workspace | Wedge | Bundle | Blocking scoreboard row |
|---|---|---|---|---|
| `dogfood_session:external_alpha.ts_web_app.full_loop` | `refws.ts_web_app_archetype_seed` | `alpha_wedge:typescript_javascript` | `launch_bundle:typescript_web_app.seed` | `scoreboard_row:alpha_scope.ts_js_navigation` |
| `dogfood_session:external_alpha.ts_web_app.run_test_debug` | `refws.ts_web_app_archetype_seed` | `alpha_wedge:typescript_javascript` | `launch_bundle:typescript_web_app.seed` | `scoreboard_row:alpha_scope.ts_js_run_test_debug` |
| `dogfood_session:external_alpha.python_data_app.full_loop` | `refws.python_data_app_archetype_seed` | `alpha_wedge:python` | `launch_bundle:python_service_or_data_app.seed` | `scoreboard_row:alpha_scope.python_environment_tests` |
| `dogfood_session:external_alpha.python_data_app.debug_refactor` | `refws.python_data_app_archetype_seed` | `alpha_wedge:python` | `launch_bundle:python_service_or_data_app.seed` | `scoreboard_row:alpha_scope.python_debug_refactor` |

These reservations exist so the cadence log can grow without re-issuing
session ids when the wedge proof packets unblock.

## Known Issues Hit

No new daily-driver blockers were hit in Sessions 1–2. Pre-existing
known limits that bound the cadence claim:

| Known limit | Effect on this packet |
|---|---|
| `known_limit:external_alpha.reference_workspace_dry_run_synthetic_only` | Sessions execute against synthetic described-byte fixtures, not partner repositories. |
| `known_limit:external_alpha.scope.claimed_wedges_only` | Wedge coverage is held to TypeScript/JavaScript and Python; other wedges are out of scope. |
| `known_limit:external_alpha.no_raw_partner_content` | No raw partner content is admitted into this packet. |
| `known_limit:external_alpha.launch_bundle_seed_not_certified` | Bundles cited per session are seed-grade, not certified. |

Windowed startup (native window via `winit`/IME path) was not exercised in
the headless environment used for Session 1. That gap is structural to
the available environment, not a hot-path regression, and is consistent
with the baseline build health note.

## Acceptance States

- `entry_gate_clause_cited_from_single_artifact`
- `rotation_resolves_to_protected_reference_fixtures`
- `at_least_one_full_session_recorded`
- `current_known_limits_attached`
- `no_private_project_paths_referenced`
- `no_raw_user_content_admitted`

## First Consumer

The companion capture at
`artifacts/milestones/m2/captures/dogfood_cadence_validation_capture.json`
records the schema header, session identity, session result, and SHA-256
of each session's reproducible byte evidence. A follow-up consumer in
`crates/aureline-telemetry/src/onboarding/` may project the same fields
once the onboarding task-success record is extended to include the
internal-dogfood cadence row class.
