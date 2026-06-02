# Accessibility surface signoff stabilization — M4 milestone note

This is the milestone-level note for the accessibility surface signoff
stabilization lane that binds shell, tree, palette, diff, terminal, debugger,
settings, auth, and recovery to per-dimension qualification for the M4 stable
line.

The authoritative typed consumer is
`aureline_release::stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery`.
The canonical checked-in artifact is
`artifacts/release/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.json`.
The proof packet lives at
`artifacts/release/m4/stabilize-accessibility-signoff-across-shell-tree-palette-diff-terminal-debugger-settings-auth-and-recovery_proof_packet.md`.
The fixture corpus lives under
`fixtures/release/m4/stabilize-accessibility-signoff-across-shell-tree-palette-diff-terminal-debugger-settings-auth-and-recovery/`.
The validation capture lives at
`artifacts/release/captures/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery_validation_capture.json`.

## Scope

This lane governs the nine IDE surfaces that users interact with most directly:

| Surface | Keyboard | Screen reader | IME/grapheme/bidi | Zoom | High contrast | Reduced motion | Effective label |
|---------|----------|---------------|-------------------|------|---------------|----------------|-----------------|
| Shell | passed | passed | passed | passed | passed | passed | Stable |
| Tree | passed | passed | passed | passed | passed | passed | Stable |
| Palette | passed | passed | passed | passed | passed | passed | Stable |
| Diff | passed | blocked | partial | passed | passed | pending | Beta |
| Terminal | partial | blocked | passed | degraded | passed | pending | Beta |
| Debugger | passed | passed | passed | passed | passed | partial | Beta |
| Settings | passed | passed | passed | passed | passed | passed | Stable |
| Auth | passed | passed | passed | passed | passed | passed | Stable |
| Recovery | passed | pending | partial | passed | passed | blocked | Beta |

Each dimension is grounded in a fixture under `fixtures/accessibility/`. The
register protects the Stable claim: a measured gap on any dimension automatically
narrows the surface below the stable cutline. A surface may be held on an active
waiver only when the gap is documented and a remediation plan is attached.

## Downgrade behavior

Any row that loses freshness, certification, or proof narrows automatically
instead of lingering as an unearned stable promise:

- **Dimension blocked or pending** (`dimension_blocked`): at least one dimension
  is blocked or lacks evidence.
- **Stale proof packet** (`evidence_stale`): the proof packet is older than its
  freshness SLO.
- **Missing proof packet** (`evidence_missing`): no proof packet has been
  captured.
- **Expired waiver** (`waiver_expired`): the provisional signoff waiver passed
  its expiry date.
- **Missing owner sign-off** (`owner_signoff_missing`): the accessibility council
  has not signed.
- **Narrowed backing claim** (`claim_label_narrowed`): the stable claim manifest
  entry this surface backs is itself below the cutline.

## Verification

```
cargo test -p aureline-release
```

Run this from the repository root to validate the typed model against the
checked-in artifact and fixtures.
