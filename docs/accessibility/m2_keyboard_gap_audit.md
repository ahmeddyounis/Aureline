# External Alpha Keyboard Gap Audit

Status: seeded

This audit closes the first launch-critical keyboard coverage pass for
project entry, onboarding, migration review, restore, command palette,
preview-required command flows, Git source acquisition, and trust
review. It is intentionally evidence-backed: the shell help projection
reads the command registry, preset keymap rows, resolver winning-source
attribution, and conflict-review packets instead of restating shortcut
truth by hand.

Contract identity:

- `keyboard_gap_audit_contract_id:
  aureline.accessibility.external_alpha_keyboard_gap_audit`
- `keyboard_gap_audit_schema_version: 1`
- Runtime consumer:
  [`crates/aureline-shell/src/help/keyboard_gap_audit.rs`](../../crates/aureline-shell/src/help/keyboard_gap_audit.rs)
- Reachable help surface:
  [`crates/aureline-shell/src/help/keybinding_inspector.rs`](../../crates/aureline-shell/src/help/keybinding_inspector.rs)
- Protected fixture:
  [`fixtures/accessibility/m2_keyboard_paths/launch_keyboard_path_matrix.yaml`](../../fixtures/accessibility/m2_keyboard_paths/launch_keyboard_path_matrix.yaml)

## Source Anchors

- UI/UX Spec Sections 6.8-6.17 require Start Center, restore,
  onboarding, import, clone/open, and first-useful-work routes to remain
  distinct, keyboard reachable, and honest about trust or setup.
- UI/UX Spec Section 7 requires one command graph with keyboard routes,
  disabled reasons, palette visibility, command ids, preview posture, and
  keybinding resolver attribution.
- UI/UX Spec Sections 10.5 and 23 require durable activity/focus
  evidence, keyboard journey scripts, focus-return drills, and
  verification packets for launch-critical flows.
- UX Style Guide Sections 12.8, 14.11, and 16.18 require Start Center,
  onboarding, and durable job/activity rows to stay keyboard reachable
  and command-backed.
- Technical Design Section 7.1 requires shell, palette, keybinding, and
  restore state to share the command graph and resolver output.
- Technical Design Section 12.3 assigns Start Center, recent work,
  workspace switching, activity center, modal/keymap diagnostics, and
  accessibility tree responsibilities to the shell and persistence lanes.

## Claimed Paths

The runtime audit currently marks these launch-critical paths as covered
when their command ids are present in the registry, have a preset-backed
winning shortcut, and include a focus-return state:

| Surface | Command exposure | Focus return |
|---|---|---|
| Start Center entry actions | `cmd:workspace.open_folder`, `cmd:workspace.clone_repository`, `cmd:workspace.import_profile`, `cmd:workspace.restore_from_checkpoint` | `returned_exact` |
| First-run no-account path | `cmd:workspace.open_folder`, `cmd:workspace.import_profile` | `returned_exact` |
| Migration import review | `cmd:workspace.import_profile` | `returned_current_batch_or_detail_owner` |
| Palette diagnostics | `cmd:command_palette.open`, `cmd:labs.open_command_trace` | `returned_exact` |
| Preview-required command flow | `cmd:workspace.import_profile` | `returned_current_batch_or_detail_owner` |
| Restore and recovery handoff | `cmd:workspace.restore_from_checkpoint` | `returned_placeholder_announced` |
| Git source acquisition review | `cmd:workspace.clone_repository` | `returned_current_batch_or_detail_owner` |

## Remaining Gaps

These are tracked as actionable gaps, not qualitative notes:

| Surface | Current state | Required follow-up |
|---|---|---|
| Git status, stage, and commit baseline | No command-backed alpha route is registered yet. | Seed source-control command descriptors for status, stage, unstage, commit, and open-diff, then attach resolver-backed shortcuts and focus-return fixtures. |
| Trust and restricted-mode review | The native shell demo has a keyboard trust-state toggle, but no trust-review command descriptor is registered yet. | Promote trust review, trust elevation, continue restricted, and open trust details into command descriptors before claiming full trust-surface parity. |

Enterprise rollout and fleet-depth keyboard proof is an explicit
non-goal for this audit. The local restricted-mode baseline above is
the trust surface this lane covers.

## Resolver Evidence

The help projection consumes:

- `aureline_input::presets::preset_binding_rows` for active shortcut
  attribution;
- `aureline_input::presets::preset_conflicts` for conflict reporting;
- `aureline_input::keybindings::KeybindingResolver::resolve` for
  winning source, winning layer, sequence state, and resolution kind.

Preset coverage is required for VS Code, IntelliJ, Vim, and Emacs
profiles. The fixture also requires the Vim conflict packet for
`Ctrl+Shift+Y`, which exercises conflict reporting instead of silently
masking a same-layer collision.

## Verification

Run:

```sh
cargo test -p aureline-shell --test keyboard_gap_audit
```

The test verifies that every claimed path has a route or explicit gap,
focus return is recorded, the preview-required import flow is present,
all preset profiles cover the command-backed path set, resolver source
attribution is populated, and remaining Git/trust gaps are actionable.
